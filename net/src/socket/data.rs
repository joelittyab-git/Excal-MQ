/// Represents different types of data with their associated values.
///
/// This enum is used to represent data in various formats, including
/// text encodings and raw binary data. It helps handle different data 
/// types by encapsulating them in a single enum type.
///
/// # Variants
///
/// - `Utf8(String)`:
///   - Represents data encoded in UTF-8. This variant contains a `String`
///     which holds the text data after deserialization. UTF-8 is a variable-length
///     encoding that supports a wide range of characters and is backward compatible
///     with ASCII.
///
/// - `Utf16(String)`:
///   - Represents data encoded in UTF-16. This variant contains a `String`
///     which holds the text data after deserialization. UTF-16 is a variable-length
///     encoding where most characters are represented by 2 bytes, and characters
///     outside the Basic Multilingual Plane (BMP) are represented by surrogate pairs.
///
/// - `Bytes(Vec<u8>)`:
///   - Represents raw binary data. This variant contains a `Vec<u8>`
///     which holds the raw bytes being transmitted. This is used when dealing
///     with non-textual binary data.
///
/// # Example
///
/// ```
/// use your_crate::Data;
///
/// let text_data = Data::Utf8("Hello, world!".to_string());
/// match text_data {
///     Data::Utf8(s) => println!("UTF-8 Encoded Text: {}", s),
///     Data::Utf16(s) => println!("UTF-16 Encoded Text: {}", s),
///     Data::Bytes(b) => println!("Raw Bytes: {:?}", b),
/// }
/// ```
///
/// This enum allows for handling different types of data in a unified manner,
/// making it easier to work with textual and binary data in your application.
pub enum Data {
     /// UTF-8 Encoding, contains the string after deserialization
     Utf8(String),
 
     /// UTF-16 Encoding, contains the string after deserialization
     Utf16(String),
 
     /// Raw bytes being transmitted 
     Bytes(Vec<u8>),
 }
 

/// Represents different types of data encoding or formats.
///
/// This enum is used to specify the encoding or format of data, typically 
/// after deserialization. It can be used to handle different types of data 
/// representations, such as text encoding or raw binary data.
///
/// # Variants
///
/// - `Utf8`:
///   - Represents UTF-8 encoding. This variant indicates that the data is
///     encoded using UTF-8 and will contain the string after deserialization.
///     UTF-8 is a variable-length encoding that supports a wide range of characters
///     and is backward compatible with ASCII.
///
/// - `Utf16`:
///   - Represents UTF-16 encoding. This variant indicates that the data is
///     encoded using UTF-16 and will contain the string after deserialization.
///     UTF-16 is a variable-length encoding where most characters are represented
///     by 2 bytes, and characters outside the Basic Multilingual Plane (BMP) are
///     represented by surrogate pairs.
///
/// - `Bytes`:
///   - Represents raw bytes. This variant indicates that the data is not
///     necessarily in a textual encoding but is simply raw binary data.
///
/// # Example
///
/// ```
/// use your_crate::Type;
///
/// let data_type = Type::Utf8;
/// match data_type {
///     Type::Utf8 => println!("Data is encoded in UTF-8"),
///     Type::Utf16 => println!("Data is encoded in UTF-16"),
///     Type::Bytes => println!("Data is raw bytes"),
/// }
/// ```
///
/// This enum helps to specify and handle different data formats, making it easier
/// to work with various types of data in your application.
pub enum Type {
     /// UTF-8 Encoding, contains the string after deserialization
     Utf8,
 
     /// UTF-16 Encoding, contains the string after deserialization
     Utf16,
     
     /// Raw bytes 
     Bytes,
 }
 

//// Represents the endianness of data.
///
/// Endianness refers to the order in which bytes are arranged within
/// a larger data type in memory. This enum is used to specify the byte
/// order for encoding and decoding data.
///
/// # Variants
///
/// - `Big`:
///   - Represents big-endian byte order. In big-endian format, the most
///     significant byte (the "big end") is stored at the lowest memory address.
///     For example, a 32-bit number `0x12345678` would be stored as `12 34 56 78`.
///
/// - `Little`:
///   - Represents little-endian byte order. In little-endian format, the least
///     significant byte (the "little end") is stored at the lowest memory address.
///     For example, a 32-bit number `0x12345678` would be stored as `78 56 34 12`.
///
/// # Example
///
/// ```
/// use your_crate::Endian;
///
/// let endian = Endian::Big;
/// match endian {
///     Endian::Big => println!("Big-endian"),
///     Endian::Little => println!("Little-endian"),
/// }
/// ```
///
/// This enum can be used to handle byte order in various contexts, such as
/// when reading or writing binary data in different formats or protocols.
pub enum Endian {
     /// Big-endian byte order.
     Big,
     /// Little-endian byte order.
     Little,
 }
 

impl Data{
     /// Converts a [std::vec::Vec<u8>] bytes to utf-16 encoded [std::vec::Vec<u16>] based on the passed [Endian]
     /// 
     /// # Arguments
     /// * `buf`: The buffer of [std::vec::Vec<u8>] bytes
     /// * `endian`: The [Endian] to encode utf16
     /// 
     /// # Returns 
     /// A [std::vec::Vec<u16>] which is utf-16 encoded
     pub async fn to_utf16_encoded(buf:&Vec<u8>, endian:Endian)->Vec<u16>{
          //buffer in utf16
          let mut u16_buf = Vec::new();
          let buf_length = buf.len();

          //itteration to convert utf8 bytes to utf16 bytes by 2 chunks
          for i in (0..buf_length).step_by(2){
               if i+1< buf_length{

                    let s:u16;     //16 byte char

                    //conversion by endian
                    match endian {

                         Endian::Big=>{
                              s = u16::from_be_bytes([buf[i], buf[i+1]]);
                         },
                         Endian::Little=>{
                              s = u16::from_le_bytes([buf[i], buf[i+1]]);
                         }
                    }
                    u16_buf.push(s);
               }
          }

          u16_buf
     }

     /// Converts a [std::vec::Vec<u8>] bytes to utf-16 encoded [std::string::String] 
     /// 
     /// # Arguments
     /// * `buf`: The buffer of [std::vec::Vec<u8>] bytes
     /// * `endian`: The [Endian] to encode utf16
     /// 
     /// # Returns 
     /// A [std::string::String] which is utf-16 encoded
     pub async fn to_utf16_string(buf: &Vec<u8>, endian:Endian)->String{
          let utf16_encoded = Self::to_utf16_encoded(buf, endian).await;

          String::from_utf16_lossy(&utf16_encoded)
     }
}