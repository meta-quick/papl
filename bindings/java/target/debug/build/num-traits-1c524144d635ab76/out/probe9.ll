; ModuleID = 'probe9.cfedfaaaa795027-cgu.0'
source_filename = "probe9.cfedfaaaa795027-cgu.0"
target datalayout = "e-m:o-i64:64-i128:128-n32:64-S128"
target triple = "arm64-apple-macosx11.0.0"

; core::f64::<impl f64>::to_ne_bytes
; Function Attrs: inlinehint uwtable
define internal i64 @"_ZN4core3f6421_$LT$impl$u20$f64$GT$11to_ne_bytes17h40843d5513a2d094E"(double %self) unnamed_addr #0 {
start:
  %_0 = alloca [8 x i8], align 1
  %self1 = bitcast double %self to i64
  store i64 %self1, ptr %_0, align 1
  %0 = load i64, ptr %_0, align 1
  ret i64 %0
}

; probe9::probe
; Function Attrs: uwtable
define void @_ZN6probe95probe17haac3d0bae107edbeE() unnamed_addr #1 {
start:
  %0 = alloca i64, align 8
  %_1 = alloca [8 x i8], align 1
; call core::f64::<impl f64>::to_ne_bytes
  %1 = call i64 @"_ZN4core3f6421_$LT$impl$u20$f64$GT$11to_ne_bytes17h40843d5513a2d094E"(double 3.140000e+00)
  store i64 %1, ptr %0, align 8
  call void @llvm.memcpy.p0.p0.i64(ptr align 1 %_1, ptr align 8 %0, i64 8, i1 false)
  ret void
}

; Function Attrs: nocallback nofree nounwind willreturn memory(argmem: readwrite)
declare void @llvm.memcpy.p0.p0.i64(ptr noalias nocapture writeonly, ptr noalias nocapture readonly, i64, i1 immarg) #2

attributes #0 = { inlinehint uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #1 = { uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #2 = { nocallback nofree nounwind willreturn memory(argmem: readwrite) }

!llvm.module.flags = !{!0}
!llvm.ident = !{!1}

!0 = !{i32 8, !"PIC Level", i32 2}
!1 = !{!"rustc version 1.77.2 (25ef9e3d8 2024-04-09)"}
