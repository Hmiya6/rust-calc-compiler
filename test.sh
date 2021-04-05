#! /bin/zsh

assert() {
    expected="$1"
    input="$2"
    
    # >! forces to overwrite file (zsh)
    cargo run "$input" >! tmp.s
    gcc -o tmp tmp.s
    ./tmp
    actual="$?"
    echo "result: $actual"

    if [ "$actual" = "$expected" ]; then
        echo "$input => $actual"
    else
        echo "$input => $expected expected, but got $actual"
        exit 1
    fi
}

assert 21 "5+20-4"
assert 42 "(10+ 40)/10 * 8 + 2"
assert 9 "10-1"
# assert 2100 "(100+ 100)* 10 + 100"

echo OK
