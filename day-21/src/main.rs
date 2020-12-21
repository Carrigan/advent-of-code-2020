use itertools::Itertools;

#[derive(Debug, Clone)]
struct IngredientsList {
    ingredients: Vec<String>,
    known_allergens: Vec<String>
}

fn line_to_ingredients_list(line: &str) -> IngredientsList {
    let mut line_iter = line.split(" (contains ");
    let ingredient_string = line_iter.next().unwrap();
    let ingredients = ingredient_string
        .split(" ")
        .map(|i| String::from(i))
        .collect();

    let allergen_string = line_iter.next().unwrap();
    let known_allergens = allergen_string[..allergen_string.len() - 1]
        .split(", ")
        .map(|a| String::from(a))
        .collect();

    IngredientsList { ingredients, known_allergens }
}

fn find_overlap(list: &IngredientsList, other: &IngredientsList) -> IngredientsList {
    let ingredient_overlap = list.ingredients
        .iter()
        .filter(|ingredient| other.ingredients.contains(ingredient))
        .map(|ingredient| ingredient.clone())
        .collect();

    let allergen_overlap = list.known_allergens
        .iter()
        .filter(|allergen| other.known_allergens.contains(allergen))
        .map(|allergen| allergen.clone())
        .collect();

    IngredientsList { ingredients: ingredient_overlap, known_allergens: allergen_overlap }
}

fn common_ingredients_by_allergen(lists: &Vec<IngredientsList>) -> Vec<IngredientsList> {
    let allergens: Vec<String> = lists.iter()
        .map(|l| l.known_allergens.clone())
        .flatten()
        .unique()
        .collect();

    allergens.iter().map(|allergen| {
        let mut allergen_lists = lists.iter()
            .filter(|l| l.known_allergens.contains(allergen));

        let first_list = allergen_lists.next().unwrap().clone();
        allergen_lists.fold(first_list, |comb, list| {
            find_overlap(&comb, list)
        })
    }).collect()
}

fn solve_allergens(commonalities: Vec<IngredientsList>) -> Vec<(String, String)> {
    let mut solved: Vec<(String, String)> = Vec::new();
    let allergen_count = commonalities.len();

    while solved.len() < allergen_count {
        let mut new_finds: Vec<(String, String)> = commonalities.iter()
            .filter(|l|
                l.ingredients.iter().filter(|i| solved.iter().find(|(ing, _)| &ing == i).is_none()).count() == 1
            )
            .map(|l|
                (
                    l.ingredients.iter().find(|i| solved.iter().find(|(ing, _)| &ing == i).is_none()).unwrap().clone(),
                    l.known_allergens.first().unwrap().clone()
                )
            )
            .collect();

        solved.append(&mut new_finds);
    }

    solved
}

fn non_allergen_count(foods: &Vec<IngredientsList>, allergenic_foods: Vec<String>) -> usize {
    foods
        .iter()
        .map(|l| l.ingredients.iter().filter(|i| !allergenic_foods.contains(i)).count())
        .sum()
}

fn main() {
    let foods: Vec<IngredientsList> = std::fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|l| line_to_ingredients_list(l))
        .collect();

    let commonalities = common_ingredients_by_allergen(&foods);
    let solved_allergens = solve_allergens(commonalities);
    let allergenic_indredients = solved_allergens.iter().map(|(ing, _)| ing.clone()).collect();

    println!("Part one: {:?}", non_allergen_count(&foods, allergenic_indredients));
}

#[test]
fn test_part_one() {
    let foods: Vec<IngredientsList> = std::fs::read_to_string("example1.txt")
        .unwrap()
        .lines()
        .map(|l| line_to_ingredients_list(l))
        .collect();

    let commonalities = common_ingredients_by_allergen(&foods);
    let solved_allergens = solve_allergens(commonalities);
    let allergenic_indredients = solved_allergens.iter().map(|(ing, _)| ing.clone()).collect();
    assert_eq!(non_allergen_count(&foods, allergenic_indredients), 5);
}
