; Memory access, numbers are number of bytes
; LD = LD4

SET R1 0x03
SET R2 0x04
MOV R0 R31
SET R3 @Function1
CALL R3 ; Call function1(3, 4)

SET R1 0x13
SET R2 0x37
MOV R0 R31
SET R3 @Function1
CALL R3; Call function1(13, 37)
SET R3 @loop
CALL R3; Call function1(13, 37)
J R3

@Function1
; use R0 as return, R1 as arg1, R2 as arg2 and they are caller-saved registers
; Store t as stack variable

; i32 function1(i32 x, i32 y)
;   let i32 t = 0x0000c0de
;   return x*t + y

; Store t
SET R3 0x04
SUB R3 R3 R31
SET R3 0x0000c0de
ST4 R31 R3

; Calculate x*t + y
LD4 R0 R31
MUL R0 R0 R1
ADD R0 R0 R2
SET R3 0x04
ADD R31 R31  R3
RET
; Function 1 end

@loop
SET R3 @loop
J R2
