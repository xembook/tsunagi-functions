#! /usr/bin/python3
# jsで記述されたテストを

import re

# tsunagi-sdk/rustディレクトリにて実行する事を想定
# 相対パスにて管理
SRC_TEST_FILE = "../test-0.1.html"
OUT_TEST_FILE = "./tests/a.rs"

header = '''
use tsunagi_sdk::*;
use json::{self, object, JsonValue};

fn get_network_info() -> JsonValue {
    let network = object!{
        version:1,
        network:"TESTNET",
        generationHash:"7fccd304802016bebbcd342a332f91ff1f3bb5e902988b352697be245f48e836",
        currencyMosaicId:0x3A8416DB2D53B6C8u64,
        currencyNamespaceId:0xE74B99BA41F4AFEEu64,
        currencyDivisibility:6,
        epochAdjustment:1637848847,
        catjasonBase:"https://xembook.github.io/tsunagi-sdk/catjson/",
        wellknownNodes:[
            "https://sym-test.opening-line.jp:3001",
            "https://sym-test.opening-line.jp:3001",
            "https://sym-test.opening-line.jp:3001",
        ]
    };
    network
}

fn get_deadline(network: &JsonValue) -> u64 {
    let now = network["epochAdjustment"].as_u64().unwrap();
    let deadline = ((now + 7200) - network["epochAdjustment"].as_u64().unwrap()) * 1000;
    deadline
}

fn get_payload(tx: &JsonValue) -> String {
    let network = get_network_info();
    let catjson = load_catjson(&tx, &network);
    let layout = load_layout(&tx, &catjson, false);
    let prepared_tx = prepare_transaction(&tx, &layout, &network);
    let parsed_tx = parse_transaction(&prepared_tx, &layout, &catjson, &network);
    let built_tx = build_transaction(&parsed_tx);
    let signature = sign_transaction(&built_tx, private_key, &network);
    let built_tx = update_transaction(&built_tx, "signature", "value", &signature);

    let _tx_hash = hash_transaction(&tx["signer_public_key"].to_string(), &signature, &built_tx, &network);
    let payload = hexlify_transaction(&built_tx.into(), 0);
    payload
}

const private_key: &str = "94ee0f4d7fe388ac4b04a6a6ae2ba969617879b83616e4d25710d688a89d80c7";
const bob_private_key: &str = "fa6373f4f497773c5cc55c103e348b139461d61fd4b45387e69d08a68000e06b";
const carol_private_key: &str = "1e090b2a266877a9f88a510af2eb0945a63dc69dbce674ccd83272717d4175cf";
\n
'''

delete_tasks = (
    "<[^\n]+>", # html部分
    "jasmine.DEFAULT_TIMEOUT_INTERVAL[= ]+[0-9]+;",
    "beforeEach\(async\(\)=>\{[\n\t ]*\}\);",
    "afterEach\(async\(\)=>\{[\n\t ]*\}\);",
    "console.log\(.*\);",
    "console.log\(.*\)",
    "await "
)

replace_and_convert_tasks = (
    ("describe\('([a-zA-Z0-9 \.\-\_]+)',[ ]*\(\) => {", "#[cfg(test)]\nmod " + r"\1 {" + "\nuse super::*;"),
    ("it\('([a-zA-Z0-9 _]+)', async \(\) => {", "#[test]\nfn " + r"test_\1() {"),
    ("\)[\n\t ]*.toEqual\(", ", "),
    ("([a-z0-9]+[A-Z]+[a-zA-Z0-9]+)\(", r"\1("),
    ("this\.([a-zA-Z]*rivateKey)", r"\1"),
)

replace_tasks = (
    ("let ([a-zA-Z0-9]+) = {", r"let \1 = object!{"),
    ("([a-zA-Z0-9]+)[ ]*=>([^\)]+)", r"|&\1| \2).unwrap("),
    ("'", "\""),
    ("expect\(", r"assert_eq!("),
    ("}\n", "};\n"),
    ("\}\);", "}"),
    ("\}\)", "}"),
    ("===", "=="),
    ("this.network", "&get_network_info()"),
    ("(\([\w, &]*)(tx[0-9]*[,\)])", r"\1&\2"),
    ("(catjson[0-9]*[,\)])", r"&\1"),
    ("\.layout", '["layout"]'),
    ("(\W)(layout[0-9]*[,\)])", r"\1&\2"),
    ("(\W)(aggTx[0-9]*[,\)])", r"\1&\2"),
    ("(\W)(preparedTx[0-9]*[,\)])", r"\1&\2"),
    ("(\W)(parsedTx[0-9]*[,\)])", r"\1&\2"),
    ("(\W)(builtTx[0-9]*[,\)])", r"\1&\2"),
    ("(\W)(signature[0-9]*[,\)])", r"\1&\2"),
    (".find\(", ".iter().find("),
    ("\.name", '["name"]'),
    ("\.signature", '["signature"]'),
    ("\.cosignatures", '["cosignatures"]'),
    ("\.signer_public_key", '["signer_public_key"]'),
    ("this\.deadline", 'get_deadline(this.network)'),
    ("this.network", "&get_network_info()"),
    ("([^&])network\)", r"\1&get_network_info())"),
    ("this.get_payload", "&get_payload"),
    ("\.length", ".len()"),
    ("(hexlify_transaction)\(([^,\)]+)\)", r"\1(\2, 0)"),
    ("(hexlify_transaction\([^,\)]+)(, [^,\)]+\))", r"\1.into()\2"),
    ("(generate_namespace_id)\(([^,\)]+)\)", r"\1(\2, 0)"),
    ("(count_size)\(([^,\)]+)\)", r"\1(\2, 0)"),
    ("(generate_namespace_id)\(([^,\)]+)\)", r"\1(\2, 0)"),
    ("([0-9])n", r"\1u64"),
    ("(0x[0-9A-F]+)n", r"\1u64"),
    ("(cosign_transaction\([^;]+);", r"\1.into();"),
    ("\[cosignaturesLayout\]", r"must_json_array_as_ref(cosignaturesLayout)"),
    ("(aggTx\[\"signer_public_key\"\])", r"&\1.to_string()"),
    ("(parsedCosignatures\[0\]\[\"layout\"\])", r"&\1.to_string()"),
    ("(txHash,)", r"&\1"),
)


def main():
    # javascriptのファイルを読み込む
    with open(SRC_TEST_FILE, "r") as f:
        data = f.read()

    for task in delete_tasks:
        data = re.sub(task, "", data)
    target = "beforeAll("
    depth = 1
    start_i = data.find(target)
    i = start_i + len(target) + 1
    while depth > 0:
        if data[i] == "(":
            depth += 1
        elif data[i] == ")":
            depth -= 1
        i += 1
    data = data.replace(data[start_i:i+1], "")

    for task in replace_and_convert_tasks:
        while re.search(task[0], data):
            match = re.search(task[0], data)
            tmp = task[1]
            for i in range(10): # 十分大きい適当な数
                tmp_old = r"\{}".format(i+1) 
                if tmp_old in task[1]:
                    tmp = tmp.replace(tmp_old, camel_to_snake(to_rust_convention(match.group(i+1))))
                else:
                    break
            data = data.replace(match.group(0), tmp)
            print(match.group(0))

    for task in replace_tasks:
        data = re.sub(task[0], task[1], data)
    data = data.strip()

    # rustのファイルを書き出す
    with open(OUT_TEST_FILE, "w") as f:
        f.write(header + data)

def camel_to_snake(name):
    name = re.sub("(.)([A-Z][a-z]+)", r"\1_\2", name)
    return re.sub("([a-z0-9])([A-Z])", r"\1_\2", name).lower()
def to_rust_convention(name):
    return name.replace("-", "_").replace(".", "_").replace(" ", "_")






if __name__ == "__main__":
    main()