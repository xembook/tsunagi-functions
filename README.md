# tsunagi-functions
This is a bridge until a de facto standard SDK is available.

# generating cstjson

like this

```
wget https://github.com/symbol/symbol/archive/refs/tags/catbuffer/parser/v3.1.0.tar.gz
wget https://github.com/symbol/symbol/archive/refs/tags/catbuffer/schemas/v3.1.0.tar.gz
...
pip3 install -r requirements.txt --user
...
python3 -m catparser --schema ../schemas/symbol/lock_hash/hash_lock.cats  --include ../schemas/symbol -o output/hash_lock.yaml
```
