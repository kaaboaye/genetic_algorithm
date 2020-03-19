use crate::consts::Number;
use nalgebra::DVector;
use rand::distributions::{Distribution, Uniform};
use rand::prelude::thread_rng;

/// Returns DVector of zeros and ones.
/// It will contain randomly distributed `desired_positives` of ones (1).
/// The rest of values will be 0.
pub fn random_vector(desired_positives: usize, size: usize) -> DVector<Number> {
  let res = if desired_positives == 0 {
    // fast path for vector full of 0
    vec![0; size]
  } else if desired_positives == size {
    // fast path for vector full of 1
    vec![1; size]
  } else if desired_positives <= size / 2 {
    // generate sparse vector
    sparse_random_vec(desired_positives, size)
  } else {
    // generate dense vector
    //
    // In order to avoid large number of collisions create sparse negation and then
    // and then negate the vector back.
    let mut res = sparse_random_vec(size - desired_positives, size);

    for num in res.iter_mut() {
      *num = *num ^ (1 as Number);
    }

    res
  };

  DVector::<Number>::from_vec(res)
}

fn sparse_random_vec(desired_positives: usize, size: usize) -> Vec<Number> {
  // setting desired positions to zero will cause and infinite loop
  // use vec![0, size] instead
  debug_assert_ne!(desired_positives, 0);

  let mut rng = thread_rng();
  let mut res: Vec<Number> = vec![0; size];
  let mut positives: usize = 0;

  let slots = Uniform::from(0..size);

  loop {
    let idx = slots.sample(&mut rng);

    // It will always increment the number of positives at first
    // then it will subtract value of given position.
    //
    // This subtraction handles collisions.
    //
    // It is equivalent to
    // `if res[idx] == 0 { positives += 1 }`
    // but this is faster because no branching is happening in this implementation.
    positives += 1;
    positives -= res[idx] as usize;

    res[idx] = 1;

    if positives == desired_positives {
      break;
    }
  }

  res
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn random_vec_generates_empty_vec() {
    let res = random_vector(0, 10);
    // assert size
    assert_eq!(res.ncols(), 1);
    assert_eq!(res.nrows(), 10);
    // assert number of positives
    assert_eq!(res.data.as_vec().iter().filter(|x| **x == 1).count(), 0);
  }

  #[test]
  fn random_vec_generates_full_vec() {
    let res = random_vector(10, 10);
    // assert size
    assert_eq!(res.ncols(), 1);
    assert_eq!(res.nrows(), 10);
    // assert number of positives
    assert_eq!(res.data.as_vec().iter().filter(|x| **x == 1).count(), 10);
  }

  #[test]
  fn random_vec_generates_sparse_vec() {
    // since its a random function repeat it 100 times
    for _ in 0..100 {
      let res = random_vector(2, 10);
      // assert size
      assert_eq!(res.ncols(), 1);
      assert_eq!(res.nrows(), 10);
      // assert number of positives
      assert_eq!(res.data.as_vec().iter().filter(|x| **x == 1).count(), 2);
    }
  }

  #[test]
  fn random_vec_generates_dense_vec() {
    // since its a random function repeat it 100 times
    for _ in 0..100 {
      let res = random_vector(8, 10);
      // assert size
      assert_eq!(res.ncols(), 1);
      assert_eq!(res.nrows(), 10);
      // assert number of positives
      assert_eq!(res.data.as_vec().iter().filter(|x| **x == 1).count(), 8);
    }
  }
}
