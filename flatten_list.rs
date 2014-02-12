fn flatten(l: &[T]) {
	println(fmt!("%?", l));

}

fn main() {
	let lst_int: [int, ..3] = [1, 2, 3];
	/*let lst_lst: [[int, ..3], int] = [lst_int, 5];*/
	/*let lst_int: [int, ..3] = [1, 2, 3];*/
	flatten(lst_int);
/*pub fn from_vec<T:Clone + 'static>(v: &[T]) -> @List<T> {*/
    /*v.rev_iter().fold(@Nil::<T>, |t, h| @Cons((*h).clone(), t))*/
	/*let lst: mut [int, ..3] */
	/*let mut lst = ~[];*/
    /*let mut lst = ~[[1], 2, [[3,4], 5], [[[]]], [[[6]]], 7, 8, []];*/

}
