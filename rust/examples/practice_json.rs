use json;
use json::object;

fn main() {
    let parsed = json::parse(r#"

    {
        "code": 200,
        "success": true,
        "payload": {
            "features": [
                "awesome",
                "easyAPI",
                "lowLearningCurve"
            ]
        }
    }

    "#).unwrap();

    let instantiated = object!{
        // quotes on keys are optional
        "code": 200,
        success: true,
        payload: {
            features: [
                "awesome",
                "easyAPI",
                "lowLearningCurve"
            ]
        }
    };

    assert_eq!(parsed, instantiated);
    println!("{:?}", parsed["code"]);
}