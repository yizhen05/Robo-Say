use std::env;

fn main() {
    // コマンドライン引数を取得
    let args: Vec<String> = env::args().skip(1).collect();

    // 引数が指定されていない場合、エラーメッセージを表示して終了
    if args.is_empty() {
        eprintln!("使用方法: <プログラム名> <文字列>");
        std::process::exit(1);
    }

    // 入力文字列を取得
    let input = args.join(" ");

    // 入力文字列の長さを計算
    let len = input.len();

    // 吹き出しの上部と下部を作成
    let top_line = "_".repeat(len);
    let bottom_line = "-".repeat(len);

    // 吹き出しを出力
    println!(" {}", top_line);
    println!("< {} >", input);
    println!(" {}", bottom_line);

    // ロボットの出力
    println!("     \\      o");
    println!("      \\     |");
    println!("       ___________");
    println!("      /           \\");
    println!("     /   O    O    \\");
    println!("    |_______________|");
    println!("   /|      000      |\\");
    println!("  / |               | \\");
    println!("    |_______________|");
    println!("       __|     |__");
}
