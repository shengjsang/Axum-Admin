use model::response::Res;
use model::system::info::Info;

pub async fn get_info() -> Res<Info> {
    match service::system::info::get() {
        Ok(info) => Res::ok_with_data(info),
        Err(_) => Res::error(500),
    }
}
