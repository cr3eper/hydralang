





#[derive(Debug, Clone)]
pub struct Vector {
    data: Vec<f64>,
    size: usize

}

impl Vector {

    pub fn set_at(&self, index: usize, value: f64) -> Self {
        let mut result = self.clone();
        result.data[index] = value;
        result

    }

    pub fn x(&self, value: f64) -> Self {
        self.set_at(0, value)
    }

    pub fn y(&self, value: f64) -> Self {
        self.set_at(1, value)
    }

    pub fn z(&self, value: f64) -> Self {
        self.set_at(2, value)
    }

    pub fn w(&self, value: f64) -> Self {
        self.set_at(3, value)
    }

    pub fn new(size: usize) -> Self {

        Vector {

            data: vec![0.0; size],
            size: size

        }

    }

    pub fn add(&self, other: &Self) -> Self {

        let mut result = Vector::new(self.size);

        for i in 0..self.size {

            result.data[i] = self.data[i] + other.data[i];

        }

        result

    }

    pub fn negate(&self) -> Self {

        let mut result = Vector::new(self.size);

        for i in 0..self.size {

            result.data[i] = -self.data[i];

        }

        result

    }

    pub fn subtract(&self, other: &Self) -> Self {

        self.add(&other.negate())

    }

    pub fn multiply_scalar(&self, scalar: f64) -> Self {

        let mut result = Vector::new(self.size);

        for i in 0..self.size {

            result.data[i] = self.data[i] * scalar;

        }

        result

    }

    pub fn multiply_vector(&self, other: &Self) -> f64 {

        let mut result = 0.0;

        for i in 0..self.size {

            result += self.data[i] * other.data[i];

        }

        result

    }

    pub fn shrink(&self, n: usize) -> Self {
            
            let mut result = Vector::new(self.size - n);
    
            for i in 0..self.size - n {
    
                result.data[i] = self.data[i];
    
            }
    
            result
    
    }

    pub fn expand(&self, n: usize) -> Self {
            
            let mut result = Vector::new(self.size + n);
    
            for i in 0..self.size {
    
                result.data[i] = self.data[i];
    
            }
    
            result
    
    }

    pub fn dot(&self, other: &Self) -> f64 {

        let mut result = 0.0;

        for i in 0..self.size {

            result += self.data[i] * other.data[i];

        }

        result

    }

    pub fn cross(&self, other: &Self) -> Self {

        assert!(self.size == 3 && other.size == 3); // Higher dimension cross products not implemented

        let mut result = Vector::new(self.size);

        result.data[0] = self.data[1] * other.data[2] - self.data[2] * other.data[1];
        result.data[1] = self.data[2] * other.data[0] - self.data[0] * other.data[2];
        result.data[2] = self.data[0] * other.data[1] - self.data[1] * other.data[0];

        result

    }

}

impl From<Vec<f64>> for Vector {

    fn from(v: Vec<f64>) -> Self {

        Vector {

            data: v.clone(),
            size: v.len()

        }

    }

}

impl PartialEq for Vector {

    fn eq(&self, other: &Self) -> bool {
        self.size == other.size && self.data.iter().zip(other.data.iter()).all(|(a, b)| a == b)
    }

}

impl Eq for Vector {}



