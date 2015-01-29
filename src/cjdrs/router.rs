use std::collections::HashMap;
use Address;
use Route;

#[derive(Debug)]
pub struct Router {
	node_store: HashMap<Address, Route>
}

impl Router {
	pub fn new(own_ip: &Address) -> Router {
		let mut router = Router { node_store: HashMap::new() };
		router.node_store.insert(own_ip.clone(), Route::new(0b1));
		router
	}

	pub fn get_route(&self, address: &Address) -> Route {
		match self.node_store.get(address) {
			Some(route) => route.clone(),
			None => Route::new(0b10)
		}
	}
}
