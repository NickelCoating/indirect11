pub fn it_works()
{
    println!("Now used as a lib by your app.");
}

#[cfg(test)]
mod tests
{
    #[test]
    fn it_works()
    {
        assert_eq!(2 + 2, 14);
    }
}
