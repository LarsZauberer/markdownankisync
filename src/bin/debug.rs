use markdownankisync::api::{get_note_ids, get_notes_data};

fn main() {
    let ids_result = get_note_ids("deck:Japanisch::Hiragana");
    let ids = ids_result.expect("Failed to get ids");
    println!("{:?}", ids);

    let data_resp = get_notes_data(&ids);
    let data = data_resp.expect("Failed to get data");
    println!("{:?}", data);
}
