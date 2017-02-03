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

macro_rules! basic_pub_all{
        (pub struct $x:ident {
            $( pub $attr_name:ident : $attr_type:ty),*
        }) =>{

            #[derive(Copy,Clone,PartialEq)]
            pub struct $x{
                $( pub $attr_name : $attr_type),*
            }
            impl $x {
                pub fn new($($attr_name:$attr_type),*) -> Self{
                    return $x {$($attr_name:$attr_name),*};
                }
                $(pub fn $attr_name(&mut self,$attr_name:$attr_type){self.$attr_name = $attr_name;})*
            }
        };
}

macro_rules! basic_pub{
        (pub struct $x:ident {
            $( $attr_name:ident : $attr_type:ty),*
        }) =>{

            #[derive(Copy,Clone,PartialEq)]
            pub struct $x{
                $($attr_name : $attr_type),*
            }
            impl $x {
                pub fn new($($attr_name:$attr_type),*) -> Self{
                    return $x {$($attr_name:$attr_name),*};
                }
            }
        };
}

macro_rules! basic{
        (struct $x:ident {
            $( $attr_name:ident : $attr_type:ty),*
        }) =>{

            #[derive(Copy,Clone,PartialEq)]
            struct $x{
                $($attr_name : $attr_type),*
            }
            impl $x {
                pub fn new($($attr_name:$attr_type),*) -> Self{
                    return $x {$($attr_name:$attr_name),*};
                }
            }
        };
}
