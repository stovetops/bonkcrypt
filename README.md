
# bonkcrypt
#### Encrypt & Decrypt megabonk progression and stats json files
#### Works on Windows or Linux assuming saves are in their default locations

    windows %USERPROFILE%/AppData/LocalLow/Ved/Megabonk/Saves/CloudDir
    linux   $HOME/.config/unity3d/Ved/Megabonk/Saves/CloudDir

### HOWTO USE:
    (linux instructions cause I forgot how to do it on windows) 
    git clone https://github.com/stovetops/bonkcrypt.git
    cd bonkcrypt
    rustup run stable cargo build --release // if you're not using rustup, just do cargo build --release instead
    chmod +x target/release/bonkcrypt
    ./target/release/bonkcrypt

 

###### I recommend you make a manual backup of your progression.json and stats.json files (specific locations above) before attempting encryption/decryption in case of any unforeseen issues or the key and/or IV has changed without an update to this code.
