use model::common::jwt::Claims;
use model::response::Res;

pub async fn protected(claims: Claims) -> Res<Claims> {
    Res::ok_with_data(claims)
}
