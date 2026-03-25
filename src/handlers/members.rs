use axum::Json;


pub async fn get_members() -> Json<Vec<String>>{
    println!("Member method called");
    let member_list: Vec<&str> = vec!["Raja", "Ravi", "Sekar", "Velu", "Mani"];

    let members: Vec<String>=  member_list.iter().map(|name| name.to_string()).collect();

    return Json(members);
}