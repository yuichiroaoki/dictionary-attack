# dictionary-attack

## Wifi 
```
sudo iwconfig  
sudo iwlist wlo1 scan | grep ESSID
nmcli d wifi connect NAME password PASSWORD
```

## Performance Test
dictionary mode
```bash
cargo run -- -d -p --password PASSWORD
```

brute force mode
```bash
cargo run -- -p --password PASSWORD
```


## References
* https://github.com/amcsi/rust-experimenting-password-cracker
* https://linuxhint.com/3-ways-to-connect-to-wifi-from-the-command-line-on-debian/

### Word list
https://github.com/danielmiessler/SecLists

