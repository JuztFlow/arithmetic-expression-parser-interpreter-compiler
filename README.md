# Parser/Interpreter/Compiler für arithmetische Ausdrücke

Ein einfacher Parser/Interpreter/Compiler für arithmetische Ausdrücke, geschrieben in Rust. Entstanden als Projektarbeit an der Hochschule Karlsruhe. Ziel der Arbeit ist die "[Umsetzung eines/des C++ Mini Parsers/Interpreters/Compilers](https://sulzmann.github.io/SoftwareProjekt/schein-neu.html#(8)) nach Rust", weswegen im weiteren Verlauf dieser Dokumentation hauptsächlich auf die Unterschiede in Bezug auf die verwendeten Sprachmittel eingegangen wird.

## Anforderungen

- [git](https://git-scm.com/downloads)
- [Rust 1.67.0 (64-bit) oder höher](https://www.rust-lang.org/learn/get-started)

## Installation

1. Klonen Sie das Repository mit `git` in ein bestehendes Verzeichnis Ihrer Wahl:

````shell
$ git clone https://github.com/JuztFlow/arithmetic-expression-parser-interpreter-compiler.git
````

2. Wechseln Sie zum neuen Verzeichnis `arithmetic-expression-parser-interpreter-compiler` und bauen Sie das Projekt mit Cargo:

```shell
$ cd arithmetic-expression-parser-interpreter-compiler
$ cargo build
```

Cargo ist ein Werkzeug zur Entwicklung und Verwaltung von Rust-Paketen und ist standardmäßig in jeder Rust-Installation enthalten. Es übernimmt die Verwaltung von Metadaten des jeweiligen Pakets, löst und kompiliert alle nötigen Abhängigkeiten, ruft den Referenz-Compiler `rustc` mit den entsprechenden Parametern auf und sorgt für eine einheitliche Paketverwaltung durch die konsequente Einhaltung von Rust-Konventionen.

## Verwendung

Um das Programm schlussendlich ausführen zu können, wird wieder Cargo im Projektverzeichnis verwendet:

```shell
$ cargo run
```

Anschließend sollte die Ausgabe des Beispielprogramms in der Konsole zu sehen sein.

## Einführung in das Projekt und seine Ziele

Das Projekt besteht im grundlegenden aus zwei Teilen:

1. **Syntax**: Einem **Parser**, der einen **arithmetischen Ausdruck** als Eingabezeichenfolge entgegennimmt und diese mittels eines **Tokenizers** in einen **Abstrakten Syntaxbaum (AST)** umwandelt, welcher als entsprechend verschachtelter Ausdruck, oder auch **`Expression`**, zurückgegeben wird. Dieser Ausdruck kann dann auf der Konsole ausgegeben, aber auch evaluiert werden, um ein Ergebnis zu erhalten.
2. **Semantik**: Einer **stack-basierten VM**, welche eine Folge von Instruktionen oder auch **`Instructions`** entgegennimmt, die anschließend durchlaufen und ausgewertet werden, um ein Ergebnis zu liefern.

Dabei ist zu beachten, dass der eingangs erwähnte [C++ Mini Parser/Interpreter/Compiler](https://sulzmann.github.io/SoftwareProjekt/schein-neu.html#(8)) hier als Vorlage dient. Das Ziel war es also dessen Struktur sinngemäß nach Rust umzusetzen und zusätzlich diese begleitende Dokumentation inklusive Vergleichen der jeweils verwendeten Sprachmittel zu erstellen. Weiterhin wurde sichergestellt, dass beide Teilaufgaben "*SYNTAX*" und "*SEMANTIK*" der Übung zum Modul "*Softwareprojekt*" immer noch lösbar sind, indem Musterlösungen an entsprechender Stelle als Kommentare im Code bereitgestellt wurden.

## Teil 1: SYNTAX

### Arithmetischer Ausdruck

E ist ein arithmetischer Ausdruck genau dann wenn:

- E ist eine Zahl
- E ist ein zusammengesetzter Ausdruck E1+E2 oder E1*E2 wobei jeweils E1 und E2 ebenfalls arithmetische Ausdrücke sind
- E ist ein geklammerter Ausdruck (E1) wobei E1 ein arithmetischer Ausdruck ist

Um es einfach zu halten, aber auch intuitiv zu gestalten werden in diesem Projekt die Ganzzahlen 0 bis 9 sowie als Operationen entsprechend Addition und Multiplikation unterstützt. Deswegen ergibt sich folgende Grammatik (angelehnt an die [EBNF](https://de.wikipedia.org/wiki/Erweiterte_Backus-Naur-Form)):

- ```
  N := 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9
  ```

- ```
  E := E + T | T
  ```

- ```
  T := T * F | F
  ```

- ```
  F := N | (E)
  ```

Dabei werden `N` und `E` als Nicht-Terminal Symbole und `0`, `1`, ..., `9`, `(`, `)`, `+` und `*` als Terminal Symbole bezeichnet.

### Darstellung eines arithmetischen Ausdrucks in Rust vs. C++

Der ***zentrale Unterschied*** in dieser Rust Implementierung im Vergleich zur ursprünglichen C++ Umsetzung besteht in der Verwendung einer Enumeration zur Repräsentation eines arithmetischen Ausdrucks:

```rust
pub enum Expression {
    Integer(i64),
    Plus(Box<Expression>, Box<Expression>),
    Multiply(Box<Expression>, Box<Expression>),
}
```

Zunächst ist `Box<T>` eine Datenstruktur in Rust, die einen generischen Wert vom Typ `T` im Heap-Speicherbereich statt auf dem Stack speichert. Mit anderen Worten, es ist eine Art von Pointer, der auf einen Wert im Heap zeigt.

Der Vorteil von `Box<T>` besteht darin, dass es dynamische Größen ermöglicht, die nicht durch die begrenzte Größe des Stack eingeschränkt werden, was hier für die beliebige Verschachtelung von Ausdrücken zwingend notwendig ist, um sicherzustellen, dass jede ggf. verschachtelte `Expression` eine sinnvolle und stabile Speicherstelle besitzt. (vgl. `shared_ptr<T>` in C++).

Die `Expression` Enumeration hat dementsprechend also drei Ausprägungen:

1. `Integer`: hält einen mit Vorzeichen codierten 64-Bit-Ganzzahl-Wert (`i64`).
2. `Plus`: hält zwei `Expression` Werte und repräsentiert eine Additionsoperation zwischen den gehaltenen Ausdrücken.
3. `Multiply`: hält zwei `Expression` Werte und repräsentiert eine Multiplikationsoperation zwischen den gehaltenen Ausdrücken.

Insgesamt führt die Verwendung einer Enumeration statt einer Vererbungsstruktur zu einer strukturierten Darstellung des Ausdrucks mit bester Übersicht, da alle Ausprägungen an einem zentralen Ort definiert sind.

Dies ist vor allem im Vergleich zur weniger kompakten, ursprünglichen Definition als Vererbungsstruktur in C++ zu erkennen:

```c++
class Exp {
    public:
    virtual int eval() = 0;
    virtual string pretty() = 0;
};

class IntExp : Exp {
    int val;
public:
    IntExp(int _val) { val = _val; }
    int eval();
    string pretty();
};

class PlusExp : Exp {
    std::shared_ptr<Exp> e1;
    std::shared_ptr<Exp> e2;
public:
    PlusExp(std::shared_ptr<Exp> _e1, std::shared_ptr<Exp> _e2) {
        e1 = _e1; e2 = _e2;
    }
    int eval();
    string pretty();
};

class MultExp : Exp {
    std::shared_ptr<Exp> e1;
    std::shared_ptr<Exp> e2;
public:
    MultExp(std::shared_ptr <Exp> _e1, std::shared_ptr<Exp> _e2) {
        e1 = _e1; e2 = _e2;
    }
    int eval();
    string pretty();
};
```

### Ausgabe und Auswertung eines arithmetischen Ausdrucks in Rust vs. C++

Der Grund für die Verwendung einer Enumeration statt einer Vererbungsstruktur liegt insbesondere darin, dass Rust mit "*Patterns und Matching*" einen speziellen und äußerst intuitiven Syntax für den Abgleich mit Typstrukturen geschaffen hat, welcher sowohl komplexe als auch einfache Typen unterstützt. Dieser eignet sich deutlich besser für die Bewältigung der vorliegenden Aufgabe als die Verwendung des Rust Trait-Systems, welches normalerweise bei der Nachbildung vererbungsartiger Strukturen in Rust eingesetzt wird.

Rust implementiert nämlich keine Vererbung in ihrer typischen Form, bekannt aus anderen Programmiersprachen, sondern verwendet das Konzept von Komposition über Vererbung (engl. "*composition over inheritance*"). Dies bedeutet, dass man ein Objekt beschreibt, indem man andere Objekte als Member-Variablen verwendet, anstatt eine neue Klasse abzuleiten. In Rust wird das Konzept der Komposition über Vererbung durch Traits und `impl`-Blöcke unterstützt, die es ermöglichen, Funktionalität hinzuzufügen, ohne die Struktur einer Klasse zu ändern.

Trotzdem ist die Verwendung von einer Enumeration in dem vorliegenden Anwendungsfall schlicht einfacher und lesbarer als die Verwendung von Traits mit entsprechenden `impl`-Blöcken, da so, egal wie viele Ausprägungen der Ausdruck hat, nur die Implementierung eines einzelnen `impl`-Blocks notwendig ist, der die gesamte Funktionalität in Form verschiedener Methoden enthält, anstatt eines eigenständigen `impl`-Blocks pro Ausprägung mit dementsprechend vielen, quasi doppelten Methoden, die die Funktionalität dann nur getrennt abbilden können.

Es folgt die Implementierung entsprechender Methoden zur Ausgabe als String und auch zur Auswertung des Ausdrucks als Teil der zuvor gezeigten Enumeration in Rust mittels eines einzelnen `impl`-Blocks:

```rust
impl Expression {
```

- ##### Ausgabe:

```rust
pub fn to_string(self: &Expression) -> String {
    match self {
        Expression::Integer(value) => value.to_string(),
        Expression::Plus(left, right) => format!("({}+{})", left.to_string(), right.to_string()),
        Expression::Multiply(left, right) => format!("({}*{})", left.to_string(), right.to_string()),
    }
}
```

- ##### Auswertung:

```rust
pub fn evaluate(self: &Expression) -> i64 {
    match self {
        Expression::Integer(value) => *value,
        Expression::Plus(left, right) => left.evaluate() + right.evaluate(),
        Expression::Multiply(left, right) => left.evaluate() * right.evaluate(),
    }
}
```

Die jeweilige Ausprägung des Ausdrucks kann hier also als Fall der `match self { ... }` Anweisung behandelt werden, um die entsprechende Funktionalität zu realisieren, anstatt für jede Ausprägung eine eigene Implementierung der jeweiligen Methode zu erfordern, welche zuvor durch die abstrakte Basisklasse innerhalb der Vererbungsstruktur vorgegeben wurde, wie es in ursprünglichen C++ Umsetzung der Fall ist:

- ##### Ausgabe:

```c++
string IntExp::pretty() { return to_string(val); }

string PlusExp::pretty() {
    string s("(");
    s.append(e1->pretty());
    s.append("+");
    s.append(e2->pretty());
    s.append(")");
    return s;
}

string MultExp::pretty() {
    string s("(");
    s.append(e1->pretty());
    s.append("*");
    s.append(e2->pretty());
    s.append(")");
    return s;
  }
```

- ##### Auswertung:

```c++
int IntExp::eval()  { return val; }

int PlusExp::eval() { return e1->eval() + e2->eval(); }

int MultExp::eval() { return e1->eval() * e2->eval(); }
```

### Parsing mittels Tokenizer in Rust vs . C++

Während in der ursprünglichen C++ Implementierung eine eigene Umsetzung des `Maybe` Statements aus der Programmiersprache Haskell vorgenommen wurde, gibt es in Rust den Typ `Option<T>` welcher unter anderem zu dem gleichen Zweck eingesetzt wird. Dabei handelt es sich um einen Enum-Typ, welcher einen generischen Wert vom Typ `T` oder eben dessen Abwesenheit repräsentiert.

Die Enumeration hat dementsprechend also zwei Ausprägungen:

1. `Some(T)`: welches einen generischen Wert vom Typ `T` enthält

2. `None`: welches anzeigt, dass kein Wert vorhanden ist

Da es sich hier wieder um einen Enum-Typ handelt, wie schon zuvor bei der `Expression`, kann dieser ebenfalls durch die Verwendung von Pattern Matching ausgewertet werden, was es also ermöglicht, unterschiedliche Aktionen auszuführen, je nachdem, ob ein Wert vorliegt, oder nicht. Dieses Konzept wird auch im exakt gleichen, logischen Kontext eingesetzt, wie in der ursprünglichen C++ Implementierung.

Schlussendlich ergeben sich also die folgenden Rust Methoden im `Parser` zur Abbildung der vorliegenden Produktionsvorschriften der eingangs definierten Grammatik:

``` rust
impl Parser {
```

- `E := E + T | T`

```rust
// E  := T E'
fn parse_e(self: &mut Parser) -> Option<Expression> {
    let t = self.parse_t();
    return match t {
        Some(t) => self.parse_e2(t),
        None => t
    };
}

// E' := + T E' |
fn parse_e2(self: &mut Parser, left: Expression) -> Option<Expression> {
    if self.tokenizer.current_token != Token::PLUS {
        return Some(left);
    }
    self.tokenizer.next_token();
    let right = self.parse_t();
    return match right {
        Some(right) => self.parse_e2(new_plus_expression(left, right)),
        None => right
    };
}
```

- `T := T * F | F`

```rust
// T  := F T'
fn parse_t(self: &mut Parser) -> Option<Expression> {
    let f = self.parse_f();
    return match f {
        Some(f) => self.parse_t2(f),
        None => f
    };
}

// T' := * F T' |
fn parse_t2(self: &mut Parser, left: Expression) -> Option<Expression> {
    if self.tokenizer.current_token != Token::MULT {
        return Some(left);
    }
    self.tokenizer.next_token();
    let right = self.parse_f();
    return match right {
        Some(right) => self.parse_t2(new_multiplication_expression(left, right)),
        None => right
    }
}
```

- `F := N | (E)`

```rust
// F := N | (E)
fn parse_f(self: &mut Parser) -> Option<Expression> {
    match self.tokenizer.current_token {
        Token::ZERO => {
            self.tokenizer.next_token();
            return Some(new_integer_expression(0));
        },
        // [...]
        Token::NINE => {
            self.tokenizer.next_token();
            return Some(new_integer_expression(9));
        },
        Token::OPEN => {
            self.tokenizer.next_token();
            let expression = self.parse_e();
            match expression {
                Some(expression) => {
                    if self.tokenizer.current_token != Token::CLOSE {
                        return None;
                    }
                    self.tokenizer.next_token();
                    return Some(expression);
                },
                None => {
                    return expression;
                }
            }
        },
        _ => {
            return None;
        }
    }
}
```

Die Rust Implementierung des hier mehrfach verwendeten `Tokenizer` bzw. dessen interne Abstraktion `Tokenize` sind hingegen in Bezug auf die verwendeten Sprachmitteln äquivalent zur ursprünglichen C++ Umsetzung und wurden lediglich erweitert, um zusätzlich die Ganzzahlen 3 bis 9 zu unterstützen, was aber trivial ist.

Der `Tokenizer` stellt also weiterhin das aktuelle Token `current_token` als Member-Variable bereit und nutzt intern `Tokenize` welches die eigentliche Funktionalität kapselt:

``` rust
pub struct Tokenizer {
    tokenize: Tokenize,
    pub current_token: Token,
}
```

Ermöglicht wird das Weiterspringen zum nächsten Token mittels seiner `next_token()` Methode welche lediglich die `next()` Methode von `Tokenize` aufruft:

``` rust
pub fn next_token(self: &mut Tokenizer) {
    self.current_token = self.tokenize.next();
}
```

Die einzige Besonderheit liegt lediglich wieder darin, dass der `Token` Typ, den der `Tokenizer` und `Tokenize` verwenden und bereitstellen, wieder als Enum-Typ definiert ist:

``` rust
pub enum Token {
    EOS,
    ZERO,
    ONE,
    TWO,
    THREE,
    FOUR,
    FIVE,
    SIX,
    SEVEN,
    EIGHT,
    NINE,
    OPEN,
    CLOSE,
    PLUS,
    MULT,
}
```

Daher wird der `Token` durch `Tokenize` in seiner vorher erwähnten `next()` Methode, die ja vom `Tokenizer` aufgerufen wird, ebenfalls mit Pattern Matching verarbeitet, allerdings hier entsprechend eines typischen Switch-Case Statements:

``` rust
fn next(self: &mut Tokenize) -> Token {
    loop {
        if self.input.len() <= self.position {
            return Token::EOS;
        }
        let current_character = self.input.chars().nth(self.position).unwrap();
        match current_character {
            '0' => {
                self.position += 1;
                return Token::ZERO;
            }
		   // [...]
            '9' => {
                self.position += 1;
                return Token::NINE;
            }
            '(' => {
                self.position += 1;
                return Token::OPEN;
            }
            ')' => {
                self.position += 1;
                return Token::CLOSE;
            }
            '+' => {
                self.position += 1;
                return Token::PLUS;
            }
            '*' => {
                self.position += 1;
                return Token::MULT;
            }
            _ => {
                self.position += 1;
            }
        }
    }
}
```

Bei der Instanziierung des `Parser` wird also die Eingabezeichenfolge an den `Tokenizer` übergeben:

``` rust
pub fn new(input: &str) -> Parser {
    Parser {
        tokenizer: Tokenizer::new(input),
    }
 }
```

Welcher dann im Anschluss umgehend `Tokenize` instanziiert und die Eingabezeichenfolge weitergibt, um sie verarbeiten und schlussendlich das aktuelle `Token` wie zuvor gezeigt via `current_token` und `next_token()` bereitstellen zu können:

``` rust
struct Tokenize {
    input: String,
    position: usize,
}

impl Tokenize {
    fn new(input: &str) -> Tokenize {
        Tokenize {
            input: input.to_string(),
            position: 0,
        }
    }
}
```

Wobei der `Tokenizer` insgesamt auch hier Teil der `Parser` Struktur ist:

``` rust
pub struct Parser {
    tokenizer: Tokenizer,
}
```

Mittels der `parse()` Methode des `Parser` kann anschließend der ganze Vorgang angestoßen werden, wobei nach der Verarbeitung eine entsprechend geschachtelte `Expression` zurückgegeben oder eben im Falle einer ungültigen Eingabezeichenfolge, das Nichtvorhandensein der ausgewerteten `Expression` durch die Rückgabe mittels des `Option<Expression>` Typs und des `None` Wertes angezeigt wird:

``` rust
pub fn parse(self: &mut Parser) -> Option<Expression> {
    return self.parse_e();
}
```

Abschließend könnte eine beispielhafte Verwendung des `Parser` im Gesamten betrachtet also konkret so aussehen:

``` rust
fn main() {
    let expression = Parser::new("(1 + 2) * 0 + 2").parse()
    match expression {
        Some(expression) => println!("{} = {}", expression.to_string(), expression.evaluate()),
        None => println!("nothing"),
    }
}
```

## Teil 2: SEMANTIK

### **Stack-basierte VM**

Die Realisierung der stack-basierten `VM` in Rust unterscheidet sich im Wesentlichen nicht von der ursprünglichen C++ Umsetzung und ist gegeben durch folgende Struktur:
``` rust
pub struct VM {
    instructions: Vec<Instruction>,
    stack: Vec<i64>,
}
```

Neben dem namensgebenden und obligatorischen Stack kapselt die VM eine Folge von Instruktionen welche selbst folgendermaßen definiert sind:
``` rust
pub struct Instruction {
    op_code: OpCode,
    value: i64,
}
```

Wobei der `OpCode` wieder als Enumeration gegeben ist:

``` rust
enum OpCode {
    PUSH,
    PLUS,
    MULT,
}
```

Die `run()` Methode der `VM` verwendet daher auch hier wieder Pattern Matching entsprechend eines typischen Switch-Case Statements während der Verarbeitung der Folge von Instruktionen:
``` rust
pub fn run(self: &mut VM) -> Option<i64> {
    for instruction in &self.instructions {
        match instruction.op_code {
            OpCode::PUSH => self.stack.push(instruction.value),
            OpCode::PLUS => {
                let right = self.stack.pop().unwrap();
                let left = self.stack.pop().unwrap();
                self.stack.push(left + right);
            },
            OpCode::MULT => {
                let right = self.stack.pop().unwrap();
                let left = self.stack.pop().unwrap();
                self.stack.push(left * right);
            },
        }
    }
    if self.stack.len() == 0 {
        return None;
    } else {
        return self.stack.pop();
    }
}
```

Wobei die Erstellung von neuen `Instructions` über die oben verwendeten Funktionen `new_push(...)`, `new_plus(...)` und `new_mult(...)` geregelt ist, die folgendermaßen definiert sind:
``` rust
pub fn new_push(value: i64) -> Instruction {
    Instruction::new_unary(OpCode::PUSH, value)
}

pub fn new_plus() -> Instruction {
    Instruction::new_nullary(OpCode::PLUS)
}

pub fn new_mult() -> Instruction {
    Instruction::new_nullary(OpCode::MULT)
}
```

Abschließend könnte eine beispielhafte Verwendung der `VM` im Gesamten betrachtet also konkret so aussehen:

``` rust
fn main() {
    let instructions = vec![
        Instruction::new_push(2),
        Instruction::new_push(3),
        Instruction::new_push(5),
        Instruction::new_plus(),
        Instruction::new_mult(),
    ];
    let result = VM::new(instructions).run();
    match result {
        Some(result) => println!("VM stack (top): {}", result),
        None => println!("VM stack (top): empty"),
    }
}
```

## Generell relevante Konzepte in Rust:

Gegeben sei folgendes Beispiel aus dem Quellcode dieses Projekts:

``` rust
struct Tokenize {
    input: String,
    position: usize,
}

impl Tokenize {
    fn new(input: &str) -> Tokenize {
        Tokenize {
            input: input.to_string(),
            position: 0,
        }
    }
}
```

- ### "*Ownership*"

  Ist ein zentrales Konzept in Rust, das die Kontrolle und Verantwortung für die Verwendung von Ressourcen im Code beschreibt. Jeder Gegenstand in Rust hat genau einen Besitzer, der für dessen Lebensdauer und Freigabe verantwortlich ist. Wenn der Besitzer eines Gegenstands aufhört zu existieren, werden auch alle Ressourcen, die von ihm besessen werden, automatisch freigegeben.

  Im Kontext des obigen Codebeispiels bedeutet das, dass der Besitzer der Eingabezeichenfolge (d.h. der Aufrufer der Funktion `new`) weiterhin verantwortlich für dessen Verwendung ist, auch wenn eine Referenz auf diese an die `Tokenize`-Struktur ausgeliehen wurde. Die `Tokenize`-Struktur kann auf den String zugreifen, aber sie besitzt ihn nicht und kann ihn vor allem nicht freigeben.

  Das Ownership-Konzept sorgt in Rust für eine klare Verantwortung für Ressourcen und hilft, Probleme wie Speicherlecks und Überlappungen von Ressourcen zu vermeiden, die in anderen Programmiersprachen häufig vorkommen.
  

- ### "*Borrowing*"

  Ist ein Konzept in Rust, das es ermöglicht, Daten an Funktionen und Methoden zu übergeben, ohne dass diese je tatsächlich von diesen besessen werden bzw. ohne dass der eigentliche Besitzer der Daten an der aufrufenden Stelle die Kontrolle über sie verliert.

  Im obigen Codebeispiel gibt die Methode `new` eine Referenz auf einen `&str` anstelle des tatsächlichen Besitzes an die `Tokenize`-Struktur. Das bedeutet, dass die Eingabezeichenfolge nicht kopiert wird, sondern stattdessen ausgeliehen wird.

  Es ist wichtig zu beachten, dass Referenzen lediglich Zugriff auf die Daten gewähren und nicht das Eigentum daran übertragen. Dies bedeutet, dass der Besitzer der Daten weiterhin volle Kontrolle darüber hat, auch wenn sie wie oben an eine Funktion oder Methode ausgeliehen wurden.

## Zusammenfassung und Fazit

Das Projekt hat also zwei Teile: Syntax und Semantik. Der Syntax-Teil besteht aus einem Parser, der einen arithmetischen Ausdruck als Eingabezeichenfolge entgegennimmt und in einen Abstrakten Syntaxbaum (AST) umwandelt. Der AST wird als entsprechend verschachtelter Ausdruck (Expression) zurückgegeben und kann auf der Konsole ausgegeben oder evaluiert werden. Der Semantik-Teil ist eine stack-basierte VM, die eine Folge von Instruktionen entgegennimmt, die dann ausgewertet werden, um ein Ergebnis zu liefern.

Diese Rust-Implementierung ist eine Neuinterpretation eines/des C++ Mini Parsers/Interpreters/Compilers und unterscheidet sich hauptsächlich darin, dass, wo möglich, konsequent Enumerations für die Repräsentation der verschiedenen Strukturen verwendet werden, allen voran um den arithmetischen Ausdrucks abzubilden. Dies ermöglicht die Verwendung von Rusts "*Pattern und Matching*" Syntax, um vor allem Übersichtlichkeit und Lesbarkeit bestmöglich umzusetzen, und reduziert entsprechenden Overhead und Boilerplate-Code, der durch Vererbungsstrukturen typischerweise anfällt, ohne dabei die Modularität einzuschränken, da weiterhin, für jede Ausprägung bzw. jeden Fall, beliebige Funktionen aufgerufen werden können.

Zunächst hätte man annehmen können, das das Trait-System von Rust besser für diese Umsetzung der ursprünglich vererbungsartigen Struktur geeignet wäre, da aber Rusts "*Pattern und Matching*" Syntax den Abgleich mit Typstrukturen ermöglicht und dabei sowohl komplexe als auch einfache Typen unterstützt, ist es wie zuvor beschrieben hier eindeutig die bessere Wahl.

Weiterhin wurde sichergestellt, dass beide Teilaufgaben "*SYNTAX*" und "*SEMANTIK*" der Übung zum Modul "*Softwareprojekt*" immer noch lösbar sind, indem Musterlösungen an entsprechender Stelle als Kommentare im Code bereitgestellt wurden, welche ebenfalls die weitere Verwendung des"*Pattern und Matching*" Syntax erfordern.

Insgesamt lag die größte Schwierigkeit darin, mit dem fehlenden Konzept von Vererbung umzugehen und stattdessen mittels Komposition zu denken. Weiterhin war die Erkenntnis, dass der "*Pattern und Matching*" Syntax hier besser geeignet ist, als das Rust Trait-System, nicht besonders offensichtlich. Dagegen waren das Konzept von "*Ownership*" und "*Borrowing*" im Kontext dieses Projekts, aufgrund der wagen Gemeinsamkeiten zu C++, relativ simpel zu verstehen.

## Quellen

- Sulzmann, Martin: Parser/Interpreter/Compiler für arithemtische Ausdrücke (C++11).
  https://sulzmann.github.io/SoftwareProjekt/schein-neu.html#(5) ff.
  Stand: Jan. 2023 - Zuletzt abgerufen am: 31.01.2023.
- Massachusetts Institute of Technology: The Rust Programming Language.
  https://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/book/first-edition/README.html ff.
  Stand: Jan. 2023 - Zuletzt abgerufen am: 31.01.2023.
- Klabnik, Steve and Nichols, Carol; with contributions from the Rust Community: The Rust Programming Language.
  https://doc.rust-lang.org/book/title-page.html ff.
  Stand: Jan. 2023 - Zuletzt abgerufen am: 31.01.2023.

## Author

Florian Eßwein - esfl1011@h-ka.de

