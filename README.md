# MocPolyWidget

This program was created to display info about currently playing song from mocp server\
```!Note: this doesn't use the song metadata, but the actual file name```

## ScreenShots
When no server is running:  
![alt text](https://github.com/Ferryistaken/mocPolyWidgetRust/blob/master/images/mocNotRunning.jpg?raw=true)

When no music is playing:  
![alt text](https://github.com/Ferryistaken/mocPolyWidgetRust/blob/master/images/mocNoMusic.jpg?raw=true)

When the music is playing:  
![alt text](https://github.com/Ferryistaken/mocPolyWidgetRust/blob/master/images/mocPlaying%20(1).jpg?raw=true)

## Dependencies
```cargo```\
```mocp```

## Installation
Paste this inside of a terminal \
```git clone https://github.com/Ferryistaken/mocPolyWidgetRust.git```\
```cd mocPolyWidgetRust```\
```cargo build --release```\
```sudo mv target/release/mocPolyWidget /usr/bin/mocPolyWidget```

Now the executable should be in your path and ready to be used anywhere by calling ```mocPolyWidget```
