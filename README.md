
# bonkcrypt
#### Encrypt & Decrypt megabonk progression and stats json files
#### Works on Windows or Linux assuming saves are in their default locations

    windows %USERPROFILE%/AppData/LocalLow/Ved/Megabonk/Saves/CloudDir
    linux   $HOME/.config/unity3d/Ved/Megabonk/Saves/CloudDir

### HOWTO USE:
    (linux instructions cause I forgot how to do it on windows) 
    git clone https://github.com/stovetops/bonkcrypt.git
    cd bonkcrypt
    rustup run stable cargo build --release
    chmod +x target/release/bonkcrypt
    ./target/release/bonkcrypt

 

###### I recommend you make a manual backup of your progression.json and stats.json files (specific locations above) before attempting encryption/decryption in case the key or IV has changed for some reason, as that would corrupt any files you attempt to en/decrypt using the old key/iv combo.
###### As of this writing (18th Sept 2026) the key/iv combo has not changed since the latest update (1.0.69/1.0.71) in Jan 2026