mod cuda_runtime;

fn main() {
    let s = Structure { function1: something, function2: anything };
    (s.function1)();
    (s.function2)("なんでも");

    let s_c = StructureWithClosure {
        function1: Box::new(|| something()),
        function2: Box::new(|s| anything(s)),
    };

    (s_c.function1)();
    (s_c.function2)("なんでも");
}

// 関数ポインタ
// fn型は関数ポインタ型
// fn() -> ()は引数を取らず、戻り値が()の関数ポインタ型
// fn(&str) -> ()は引数が&strで、戻り値が()の関数ポインタ型
// 関数ポインタ型は関数名を指定するだけで関数を指定できる
// 軽量であるが、クロージャのように状態を持つことができない
// クロージャを使うと、関数ポインタ型よりも柔軟に関数を扱うことができる
// クロージャは関数ポインタ型をラップしたもので、関数ポインタ型を引数に取る関数に渡すことができる
struct Structure {
    function1: fn() -> (),
    function2: fn(&str) -> (),
}

struct StructureWithClosure {
    function1: Box<dyn Fn() -> ()>,
    function2: Box<dyn Fn(&str) -> ()>,
}

struct StructureWithTemplate<T> {
    function1: T,
    function2: T,
}

fn something() {
    println!("えぇ!?できるんですか!?");
}

fn anything(str: &str) {
    println!("何だって!?\n{}とも出力できるんですか!?", str);
}
