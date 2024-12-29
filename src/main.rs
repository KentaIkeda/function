fn main() {
    // Rustで変数と関数を命名する際に使用される命名規則はスネークケース
    let sample_variables = "sample_variables";
    println!("{}の様にスネークケースにしようね", sample_variables);

    // 関数を定義する際、呼び出される関数の前で定義されようが後ろで定義されようがコンパイラはチェックしない
    // どこかで定義されていればコンパイルエラーはスローされない

    // main関数外で定義されたanother_functionの呼び出し
    let x = another_function(20);
    println!("The value of x is: {}", x);

    // {}は値を返す式
    let y = {
        let x = 3;
        x + 1
    };

    // {}の中で定義されたxがyに束縛されている
    println!("The value of y is: {}", y);
}

// fnキーワードで関数を定義
// 引数がある場合は、引数に対して型を指定
// 戻り値がある場合は -> i32の様に型を明示的に記述しないとコンパイルエラーが発生する
fn another_function(n: i32) -> i32 {
    // Rustにおいて、return文の文末にはセミコロンを記述しなくてもコンパイルエラーは発生しない
    return n;
    // 関数型プログラミング的に記述するために、return文を使用せず関数の戻り値として記述することも可能
    // 例
    // n => これは、引数で受け取ったnをリターンしていることを示している
}