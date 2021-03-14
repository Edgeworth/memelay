## Model
Ideas:
 - Deal with state - runs etc

Context independent:
1. Per key penalty
 - This subsumes carpalx: Row freq, finger freq + penalties
2. Hand asymmetry
3. Bonus for being similar to qwerty - low priority

Context dependent - these could be subsumed by an RNN with enough data?
1. Runs - hand, finger, row
 - Maximize row runs - stay on the same row
 - Minimize hand runs - switch hands
 - Minimize finger runs - use different fingers
2. Inward rolling - positive penalty
3. Cost for holding a key - should be more than just pressing it once though.

Statistics:
1. Finger distribution
2. Hand asymmetry
3. Row distribution

## Layout Analysis
https://colemakmods.github.io/mod-dh/analyze.html
http://patorjk.com/keyboard-layout-analyzer/#/main
http://mkweb.bcgsc.ca/carpalx/

Carpalx notes / limitations:
1. Shift-state characters (e.g. ; and :) are always on the same key.
2. Favours home row and bottom row.
- limited use of weak fingers, like pinky and ring finger
- limited use of bottom row
- increased use of home row
- limited finger travel distance
- limited same-finger typing (e.g. uhm)
- balanced hand-use vs right-hand priority (see below)
- alternating hand-use vs rolling (see below)

## Ideas
- Shift on thumb keys
- Multiple uses for thumb keys - e.g. hold, tap, etc
- Numpad on layer.
- Arrow keys same key as vim keys?
- home row modifier keys, vs thumb keys used for layers?
 - https://precondition.github.io/home-row-mods#what-are-home-row-mods
- Dedicated copy paste keys? hold copy to cut?
- Long tap or double tap to access symbols etc?
- Consider left hand kbd + right hand mouse usage
- one shot modifiers easier than chords; + double tap to stick, third to unstick.
- leds at top to signal status; leds behind keys to give hints.
- leader keys - tap and then looks at a sequence of keys
- qwerty layer: for games and guests.

## Existing layouts
colemak, colemak dhm
https://github.com/ColemakMods/mod-dh
- Colemak has more pinkie usage than qwerty according to carpalx.
- Preference to the right hand

minimize pinkie guy:
https://www.reddit.com/r/ergodox/comments/jjk0rf/moonlander_planck_and_new_layout_that_minimizes/

miryoku - minimal ortho layout:
https://github.com/manna-harbour/qmk_firmware/blob/miryoku/users/manna-harbour_miryoku/miryoku.org
https://www.reddit.com/r/ErgoDoxEZ/comments/ijtcq1/miryoku_layout_for_moonlander_planck_ez_and/

minimized pinkies:
https://github.com/t00mietum/keyboard

has coding layer:
https://github.com/naps62/ergodox-layout

34 key:
https://configure.ergodox-ez.com/ergodox-ez/layouts/GZOa/latest/0

https://github.com/qmk/qmk_firmware/tree/master/layouts/community/ergodox
