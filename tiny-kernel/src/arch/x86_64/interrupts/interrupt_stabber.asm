%macro ISR_NOERRCODE 1
  global isr%1
  isr%1:
    cli                         ; Disable interrupts firstly.
    push byte 0                 ; Push a dummy error code.
    push byte %1                ; Push the interrupt number.
    jmp isr_common_stub         ; Go to our common handler code.
%endmacro

; --- GENERATION FOR HARDWARE INTERRUPTS (32-255) ---
; HI DO NOT HAVE AN ERROR CODE, 

%assign i 32
%rep 192  ; (224 last fn)
    ISR_NOERRCODE i
%assign i i + 1
%endrep