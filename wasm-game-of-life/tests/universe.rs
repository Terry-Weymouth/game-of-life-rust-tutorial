use wasm_game_of_life::Universe;

#[test]
fn new_universe(){
    let universe = Universe::new();
    println!("{:?}", universe.render());
}