///utils
pub fn clamp<T>(v1: T,min: T,max:T)-> T where T: PartialOrd{
        if v1 > max{
            return max;
        }
        if v1 < min {
            return min;
        }
        return v1;
}
