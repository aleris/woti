Current:

```
 
 UTC  UTC  +0                       10:18  20 21
 Test                         Sat, Apr 25  
                                                
 Kathmandu  +0545  +5:45            16:03   2  3
 Nepal                        Sat, Apr 25  ⁴⁵ ⁴⁵
                                                
 San Jose  PDT  -7                3:18 AM   1  2
 United States, California    Sat, Apr 25  pm pm
                                               
 Bucharest  EEST  +3                13:18  23  0
 Romania                      Sat, Apr 25  
                                            26
 Bangalore  IST  +5:30            3:48 PM   1  2
 India                        Sat, Apr 25  ½a ½a

```


With 15 min intervals:
```
 UTC  UTC  +0                       10:18  20  ·  ·  ·  21
 Test                         Sat, Apr 25     ¹⁵ ³⁰ ⁴⁵
                                                          
 Kathmandu  NPT  +5:45              16:03   2  ·  ·  ·  3 
 Nepal                        Sat, Apr 25  ⁴⁵ ⁰⁰ ¹⁵ ³⁰ ⁴⁵
                                                          
 San Jose  PDT  -7                3:18 AM   1  ·  ·  ·  2 
 United States, California    Sat, Apr 25  pm ¹⁵ ³⁰ ⁴⁵ pm 
                                                        
 Bucharest  EEST  +3                13:18  23  ·  ·  ·  0 
 Romania                      Sat, Apr 25     ¹⁵ ³⁰ ⁴⁵
                                           
 Bangalore  IST  +5:30            3:48 PM   1  ·  ·  ·  2 
 India                        Sat, Apr 25  ½a ⁴⁵ ⁰⁰ ¹⁵ ½a 

```

Sub-row rendering rules:

- **Whole-hour 24h zones (UTC, Bucharest)**: hour cells stay blank — the
  hour digit on the row above already marks the slot. Intermediates land on
  `:15`/`:30`/`:45` (never `:00`), so no `⁰⁰` ever appears.
- **Fractional-offset zones (Nepal `+5:45`, India `+5:30`)**: hour cells
  keep their existing glyph (`⁴⁵`, `³⁰`, `½a`, `½p`, etc.). The intermediate
  cell that lands on `:00` shows `⁰⁰` so the sub-row marks the natural
  wall-hour boundary that the `·` tick on the row above does not.