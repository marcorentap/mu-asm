; Memory access, numbers are number of bytes
; LD = LD4

SET R1 0x03
SET R2 0x04
SET R0 RIP
CALL @Function1 ; Call function1(3, 4)

SET R1 0x13
SET R2 0x37
SET R0 RIP
CALL @Function1 ; Call function1(13, 37)
J @loop

@Function1
; use R0 as return, R1 as arg1, R2 as arg2 and they are caller-saved registers
; Store t as stack variable

; i32 function1(i32 x, i32 y)
;   let i32 t = 0x0000c0de
;   return x*t + y

; Store t
SUB RSP 0x04
ST4 RSP 0x0000c0de

; Calculate x*t + y
LD4 R0 RSP
MUL R0 R1
ADD R0 R2
ADD RSP 0x04
RET
; Function 1 end

@loop
J @loop
