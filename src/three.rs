trait Three{
	fn get(&self) -> (i64, i64, i64);
	fn dot<T: Three(&self, &o : T) -> i64;
	fn cross<T: Three(%self, &o : T) -> (i64, i64, i64);
}