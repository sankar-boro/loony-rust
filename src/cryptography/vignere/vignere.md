# Vigenère cipher

The Vigenère cipher is a classic method of encrypting plaintext using a simple form of polyalphabetic substitution. It was invented by the French diplomat and cryptographer Blaise de Vigenère in the 16th century, although it was later misattributed to Giovan Battista Bellaso, an Italian cryptographer.

The Vigenère cipher is considered an improvement over the more straightforward Caesar cipher, which is a type of monoalphabetic substitution cipher. In the Caesar cipher, each letter in the plaintext is shifted a fixed number of positions in the alphabet to get the corresponding ciphertext letter. However, this method is susceptible to frequency analysis, where the repeated patterns in the ciphertext can reveal the key and break the encryption.

The Vigenère cipher avoids this vulnerability by using a keyword or keyphrase to determine the shifting value for each letter in the plaintext. The keyword is repeated to match the length of the plaintext, and then the shift for each letter is determined based on the corresponding letter in the keyword. This creates a series of Caesar ciphers, each using a different shift value.

Here's a step-by-step explanation of the Vigenère cipher encryption process:

1. Key Generation: Choose a keyword or keyphrase to encrypt the plaintext. For example, the keyword could be "KEY."

2. Repeat the Key: Repeat the keyword as necessary to match the length of the plaintext. For example, if the plaintext is "HELLO," the keyword is repeated to "KEYKE."

3. Conversion: Convert each letter of the plaintext and the repeated keyword into numerical values using a standard conversion (e.g., A=0, B=1, ..., Z=25).

4. Encryption: Add the numerical values of each corresponding plaintext letter and keyword letter modulo 26 (to keep the result within the range of the alphabet). This produces the ciphertext.

5. Ciphertext: Convert the numerical ciphertext values back into letters using the reverse of the conversion used in step 3.

To decrypt the Vigenère cipher, the recipient must know the keyword used for encryption. They repeat the keyword to match the length of the ciphertext and then perform the reverse process by subtracting the numerical values of the corresponding ciphertext letter and keyword letter, modulo 26.

While the Vigenère cipher was once considered quite secure, it can still be broken using various cryptanalysis techniques, especially if the key is relatively short or weak. One of the most famous methods for breaking the Vigenère cipher is the Kasiski examination, which looks for repeating patterns in the ciphertext to infer the length of the keyword. Once the keyword length is known, statistical analysis can help determine the individual letters of the keyword and ultimately decrypt the message.