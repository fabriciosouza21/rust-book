
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// Estrutura de tupla, são struct nomeados tuplas com valores 
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

struct AlwaysEqual;

fn main() {
    // Creating a new instance of User
    let mut user1 = User {
        active: true,
        username: String::from("user1"),
        email: String::from("user1@example.com"),
        sign_in_count:1,
    };

    user1.email = String::from("anotheremail@example.com");

    let mut user2 = build_user(String::from("user2@gmail.com"), String::from("user2"));

    // Criando instâncias de outras instâncias
    let user3 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };
     /* a sintaxe de atualização da esturtura é usada  = como uma atribuição, isso ocorre por que move os dados.
     * Não podemos usar user1 como um todo após a criaçao de user2, pois user1 foi movido para user2.
     * ainda podemos utilizar os valores sign_in_count e active de user1, pois são tipos primitivos e são copiados.
     * os valores de username e email são movidos para user2.
     * CUIDADO AO USAR ESSA SINTAXE
     */
    // Creating a new instance of User
    let mut user1 = User {
        active: true,
        username: String::from("user1"),
        email: String::from("user1@example.com"),
        sign_in_count:1,
    };
    let user2 = User {
        email: String::from("another@gmail.com"),
        ..user1
    };

    /*  Usando estruturas de tupla sem campos nomeados para criar tipos diferentes
    */

    // Estrutura de tupla, são struct tuplas com valores nomeados
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    /*
    Estruturas semelhantes a unidades sem nenhum campo
    */

    // definido estutura sem campos
    let subject = AlwaysEqual;
   
}

 // Usando sintaxe simplificada de inicialização de campos
// Similar ao javascript
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}



