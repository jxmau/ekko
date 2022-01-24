-> Include the buffer in the Query struct.
    * Only the Nature::new() could return a Vec of message?

-> Should the FE call the Nature::new()? 
Front-End ask for the Nature by giving the buffer as an &mut.
Create the query with the buffer as a field but as the Object directly.
And call the main method to debug


FE - Take the Content of the file, get ride of the comments
BE - One main function: Debug.
    * Determine the nature of the Query
    * Debug the Query