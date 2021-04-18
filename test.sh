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
assert 1 "10 > 2"
assert 0 "43 == 42"
assert 1 "32 == 43 == 0"

echo OK
