struct A;
struct S(A);
struct SGen<T>(T);

fn reg_fn(_s: S) {}

fn gen_spec_t(_s: SGen<A>) {}

fn gen_spec_i32(_s: SGen<i32>) {}

fn generic<T>(_s: SGen<T>) {}

pub fn run() {
    reg_fn(__);
    gen_spec_t(__);
    gen_spec_i32(__);

    generic::<char>(__);
}