use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    // let user1 = _build_user(String::from("taro"), String::from("taro@abc.com"));
    // // 構造体更新記法で他のインスタンスからインスタンスを生成する
    // // ..という記法により、 明示的にセットされていない残りのフィールドが、与えられたインスタンスのフィールドと同じ値になるように指定します
    // let user2 = _User {
    //     username: String::from("jiro"),
    //     email: String::from("jiro@xyz.com"),
    //     ..user1 // ,は不要
    // };

    // let rec = Rectangle {
    //     width: 3,
    //     height: 4,
    // };
    // println!("menseki is {}", rec.area());
    // println!("rec is {:#?}", rec);
    // let rec2 = Rectangle {
    //     width: 2,
    //     height: 4,
    // };
    // println!("Can rec hold rec2? {}", rec._can_hold(&rec2));
    // let sq = Rectangle::_square(30);

    // let _home = _IPAddr::V4(8, 8, 8, 8);
    // let _loopback = _IPAddr::V6(String::from("::1"));
    // _home._print();
    // _loopback._print();
    _vectors()
}

fn _vectors() {
    let mut v = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
}

// Option<T>とのマッチ
fn _optiont() {
    fn _plus_one(v: Option<i32>) -> Option<i32> {
        match v {
            None => None,
            Some(n) => Some(n + 1),
        }
    }
    let _five = Some(5);
    let _six = _plus_one(_five);
    let _none = _plus_one(None);

    // if letで簡潔なフロー制御
    // 一つのケースにしか興味がないような場面では、match式はちょっと長ったらしすぎます。
    let some_u8_value = Some(0u8);
    // match some_u8_value {
    //     Some(3) => println!("three"),
    //     _ => (), // どんな値にもマッチ => 何もしない
    // }
    // if letという記法は等号記号で区切られたパターンと式を取り、式がmatchに与えられ、パターンが最初のアームになったmatchと、 同じ動作をします。
    if let Some(3) = some_u8_value {
        println!("three");
    }
    // しかしながら、 matchでは強制された包括性チェックを失ってしまいます。
    // matchかif letかの選択は、 特定の場面でどんなことをしたいかと簡潔性を得ることが包括性チェックを失うのに適切な代償となるかによります。
}

// The match Control Flow Operator
enum _Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}
#[derive(Debug)] // すぐに州を点検できるように
enum UsState {
    _Alabama,
    _Alaska,
    // ... などなど
}
fn _value_in_coin(coin: _Coin) -> u32 {
    match coin {
        // 4 arms
        _Coin::Penny => {
            println!("Lucky Penny!");
            1
        }
        _Coin::Nickel => 5,
        _Coin::Dime => 10,
        _Coin::Quarter(st) => {
            println!("State quarter from {:?}", st);
            25
        }
    }
}

// プログラムが遭遇するIPアドレスのすべての可能性. 列挙型は、取りうる値をすべて列挙でき、 これが列挙型の名前の由来
#[derive(Debug)]
enum _IPAddr {
    _V4(u8, u8, u8, u8),
    _V6(String),
}

impl _IPAddr {
    fn _print(&self) {
        println!("{:#?}", self)
    }
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn _area(&self) -> u32 {
        self.width * self.height
    }
    fn _can_hold(&self, other: &Rectangle) -> bool {
        self.height > other.height && self.width > other.width
    }
    // 関連関数
    fn _square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

// フィールドのない Unit-Like Structs
// ある型にトレイトを実装するけれども、 型自体に保持させるデータは一切ない場面に有効になります
struct _ULS {}

fn _tuple_struct() {
    // 異なる型を生成する名前付きフィールドのないタプル構造体を使用する
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);
    let _color = Color(55, 55, 55);
    let _point = Point(55, 55, 55);
    // タプル構造体のインスタンスは、 タプルと同じように振る舞います: 分配して個々の部品にしたり、.と添え字を使用して個々の値にアクセスするなどです。
}

// Structs
struct _User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn _build_user(email: String, username: String) -> _User {
    _User {
        // フィールドと変数が同名の時にフィールド初期化省略記法を使う
        username,
        email,
        sign_in_count: 1,
        active: true,
    }
}

fn _control_flow() {
    // Rustには3種類のループが存在します: loopとwhileとfor
    // loopでコードを繰り返す
    loop {
        break;
    }
    // whileで条件付きループ
    let mut i = 3;
    while i > 0 {
        i -= 1;
    }
    // forでコレクションを覗き見る
    let arr = [10, 20, 30, 40, 50];
    for ele in arr.iter() {
        println!("ele: {}", ele)
    }
    // forでカウントダウン
    for number in (1..4).rev() {
        println!("{}!", number);
    }
}

fn _data_types() {
    // shadowing
    let spaces = "   ";
    let spaces = spaces.len(); // 他の型に変えれる。hoge_strとか宣言しなくて済む
    println!("{}", spaces);

    // tuple type
    let tup = (1, 1.5, "a"); // creating
    let (_x, _y, z) = tup; // destructuring
                           // let one = tup.0;
    println!("z: {}", z);

    // let文内でif式を使う
    // ifは式なので、let文の右辺に持ってくることができます。
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("number: {}", number);
}

fn _plus_one(x: i32) -> i32 {
    x + 1
}

fn _a() {
    let _x = 5;

    // define func
    let y = {
        let _x = 3;
        _x + 1 // 式は終端にセミコロンを含みません。式の終端にセミコロンを付けたら、文に変えてしまいます
    };

    println!("The value of y is: {}", y);
}

fn _guess_number() {
    println!("guess number!");

    let secret = rand::thread_rng().gen_range(1, 101);

    // println!("secret is {}", secret);

    loop {
        println!("please input your guess!");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("failed read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("you guessed: {}", guess);

        match guess.cmp(&secret) {
            Ordering::Less => println!("Too small!"),  //小さすぎ！
            Ordering::Greater => println!("Too big!"), //大きすぎ！
            Ordering::Equal => {
                println!("You win!"); //やったね！
                break;
            }
        }
    }
}
