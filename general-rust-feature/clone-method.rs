// Cloning when Handling an Option or a Result
pub struct large_string_vec {
	// we may have a "String" in a specific position
    collection: Vec<(char, Option<String>)>,
}


// error impl
impl large_string_vec {
    pub fn error_unwrap(&mut self) -> Result<(), char> {
        for iterm in &self.collection {
            if iterm.1.is_some() {
            	// cannot move out of borrowed content
            	// use clone ? : iterm.1.cloned().unwrap().contains(iterm.0)
            	// not so good for large number of String in the vec
                if iterm.1.unwrap().contains(iterm.0) {
                    return Err(iterm.0);
                }
            }
        }
        Ok(())
    }
}


// to improve the above, first think the two functions below
// pub fn unwrap(self) -> T;
// pub fn as_ref(&self) -> Option<&T>;
impl large_string_vec {
    pub fn test_as_ref(&mut self) -> Result<(), char> {
        for iterm in &self.collection {
            if iterm.1.is_some() {
                if iterm.1.as_ref().unwrap().contains(iterm.0) {
                    return Err(iterm.0);
                }
        	}
    	}
    	Ok(())
}


// another effective way
impl large_string_vec {
    pub fn test_if_let(&mut self) -> Result<(), char> {
        for iterm in &self.collection {
            if let Some(text) = iterm.1 {
            	if text.contains(iterm.0) {
                    return Err(iterm.0);
                }
        	}
    	}
    	Ok(())
}