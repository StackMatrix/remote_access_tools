; ModuleID = 'probe1.fde74c09-cgu.0'
source_filename = "probe1.fde74c09-cgu.0"
target datalayout = "e-m:o-i64:64-i128:128-n32:64-S128"
target triple = "arm64-apple-macosx11.0.0"

%"core::fmt::Arguments<'_>" = type { { ptr, i64 }, { ptr, i64 }, { ptr, i64 } }
%"alloc::string::String" = type { %"alloc::vec::Vec<u8>" }
%"alloc::vec::Vec<u8>" = type { { i64, ptr }, i64 }

@alloc3 = private unnamed_addr constant <{}> zeroinitializer, align 8
@alloc9 = private unnamed_addr constant <{ [12 x i8] }> <{ [12 x i8] c"invalid args" }>, align 1
@alloc10 = private unnamed_addr constant <{ ptr, [8 x i8] }> <{ ptr @alloc9, [8 x i8] c"\0C\00\00\00\00\00\00\00" }>, align 8
@alloc14 = private unnamed_addr constant <{ [109 x i8] }> <{ [109 x i8] c"/Users/stack/.rustup/toolchains/nightly-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/fmt/mod.rs" }>, align 1
@alloc15 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc14, [16 x i8] c"m\00\00\00\00\00\00\00\8C\01\00\00\0D\00\00\00" }>, align 8
@alloc4 = private unnamed_addr constant <{ ptr, [8 x i8] }> <{ ptr @alloc3, [8 x i8] zeroinitializer }>, align 8
@alloc6 = private unnamed_addr constant <{ [8 x i8] }> zeroinitializer, align 8

; core::fmt::ArgumentV1::new_lower_exp
; Function Attrs: inlinehint uwtable
define { ptr, ptr } @_ZN4core3fmt10ArgumentV113new_lower_exp17h599197eddb1cfb93E(ptr align 8 %x) unnamed_addr #0 {
start:
; call core::fmt::ArgumentV1::new
  %0 = call { ptr, ptr } @_ZN4core3fmt10ArgumentV13new17h545dd2fd10a3f4acE(ptr align 8 %x, ptr @"_ZN4core3fmt3num3imp55_$LT$impl$u20$core..fmt..LowerExp$u20$for$u20$isize$GT$3fmt17h5876178a0bb85ab6E")
  %1 = extractvalue { ptr, ptr } %0, 0
  %2 = extractvalue { ptr, ptr } %0, 1
  %3 = insertvalue { ptr, ptr } undef, ptr %1, 0
  %4 = insertvalue { ptr, ptr } %3, ptr %2, 1
  ret { ptr, ptr } %4
}

; core::fmt::ArgumentV1::new
; Function Attrs: inlinehint uwtable
define { ptr, ptr } @_ZN4core3fmt10ArgumentV13new17h545dd2fd10a3f4acE(ptr align 8 %x, ptr %f) unnamed_addr #0 {
start:
  %0 = alloca ptr, align 8
  %1 = alloca ptr, align 8
  %2 = alloca { ptr, ptr }, align 8
  store ptr %f, ptr %1, align 8
  %_3 = load ptr, ptr %1, align 8, !nonnull !1, !noundef !1
  store ptr %x, ptr %0, align 8
  %_5 = load ptr, ptr %0, align 8, !nonnull !1, !align !2, !noundef !1
  store ptr %_5, ptr %2, align 8
  %3 = getelementptr inbounds { ptr, ptr }, ptr %2, i32 0, i32 1
  store ptr %_3, ptr %3, align 8
  %4 = getelementptr inbounds { ptr, ptr }, ptr %2, i32 0, i32 0
  %5 = load ptr, ptr %4, align 8, !nonnull !1, !align !2, !noundef !1
  %6 = getelementptr inbounds { ptr, ptr }, ptr %2, i32 0, i32 1
  %7 = load ptr, ptr %6, align 8, !nonnull !1, !noundef !1
  %8 = insertvalue { ptr, ptr } undef, ptr %5, 0
  %9 = insertvalue { ptr, ptr } %8, ptr %7, 1
  ret { ptr, ptr } %9
}

; core::fmt::Arguments::as_str
; Function Attrs: inlinehint uwtable
define internal { ptr, i64 } @_ZN4core3fmt9Arguments6as_str17hfbc55123107933a9E(ptr align 8 %self) unnamed_addr #0 {
start:
  %_2 = alloca { { ptr, i64 }, { ptr, i64 } }, align 8
  %0 = alloca { ptr, i64 }, align 8
  %1 = getelementptr inbounds %"core::fmt::Arguments<'_>", ptr %self, i32 0, i32 1
  %2 = getelementptr inbounds { ptr, i64 }, ptr %1, i32 0, i32 0
  %_3.0 = load ptr, ptr %2, align 8, !nonnull !1, !align !3, !noundef !1
  %3 = getelementptr inbounds { ptr, i64 }, ptr %1, i32 0, i32 1
  %_3.1 = load i64, ptr %3, align 8
  %4 = getelementptr inbounds %"core::fmt::Arguments<'_>", ptr %self, i32 0, i32 2
  %5 = getelementptr inbounds { ptr, i64 }, ptr %4, i32 0, i32 0
  %_4.0 = load ptr, ptr %5, align 8, !nonnull !1, !align !3, !noundef !1
  %6 = getelementptr inbounds { ptr, i64 }, ptr %4, i32 0, i32 1
  %_4.1 = load i64, ptr %6, align 8
  %7 = getelementptr inbounds { ptr, i64 }, ptr %_2, i32 0, i32 0
  store ptr %_3.0, ptr %7, align 8
  %8 = getelementptr inbounds { ptr, i64 }, ptr %_2, i32 0, i32 1
  store i64 %_3.1, ptr %8, align 8
  %9 = getelementptr inbounds { { ptr, i64 }, { ptr, i64 } }, ptr %_2, i32 0, i32 1
  %10 = getelementptr inbounds { ptr, i64 }, ptr %9, i32 0, i32 0
  store ptr %_4.0, ptr %10, align 8
  %11 = getelementptr inbounds { ptr, i64 }, ptr %9, i32 0, i32 1
  store i64 %_4.1, ptr %11, align 8
  %12 = getelementptr inbounds { ptr, i64 }, ptr %_2, i32 0, i32 0
  %_21.0 = load ptr, ptr %12, align 8, !nonnull !1, !align !3, !noundef !1
  %13 = getelementptr inbounds { ptr, i64 }, ptr %_2, i32 0, i32 1
  %_21.1 = load i64, ptr %13, align 8
  %_16 = icmp eq i64 %_21.1, 0
  br i1 %_16, label %bb1, label %bb3

bb3:                                              ; preds = %start
  %14 = getelementptr inbounds { ptr, i64 }, ptr %_2, i32 0, i32 0
  %_23.0 = load ptr, ptr %14, align 8, !nonnull !1, !align !3, !noundef !1
  %15 = getelementptr inbounds { ptr, i64 }, ptr %_2, i32 0, i32 1
  %_23.1 = load i64, ptr %15, align 8
  %_13 = icmp eq i64 %_23.1, 1
  br i1 %_13, label %bb4, label %bb2

bb1:                                              ; preds = %start
  %16 = getelementptr inbounds { { ptr, i64 }, { ptr, i64 } }, ptr %_2, i32 0, i32 1
  %17 = getelementptr inbounds { ptr, i64 }, ptr %16, i32 0, i32 0
  %_22.0 = load ptr, ptr %17, align 8, !nonnull !1, !align !3, !noundef !1
  %18 = getelementptr inbounds { ptr, i64 }, ptr %16, i32 0, i32 1
  %_22.1 = load i64, ptr %18, align 8
  %_7 = icmp eq i64 %_22.1, 0
  br i1 %_7, label %bb5, label %bb2

bb2:                                              ; preds = %bb4, %bb3, %bb1
  store ptr null, ptr %0, align 8
  br label %bb7

bb5:                                              ; preds = %bb1
  %19 = getelementptr inbounds { ptr, i64 }, ptr %0, i32 0, i32 0
  store ptr @alloc3, ptr %19, align 8
  %20 = getelementptr inbounds { ptr, i64 }, ptr %0, i32 0, i32 1
  store i64 0, ptr %20, align 8
  br label %bb7

bb7:                                              ; preds = %bb2, %bb6, %bb5
  %21 = getelementptr inbounds { ptr, i64 }, ptr %0, i32 0, i32 0
  %22 = load ptr, ptr %21, align 8, !align !2
  %23 = getelementptr inbounds { ptr, i64 }, ptr %0, i32 0, i32 1
  %24 = load i64, ptr %23, align 8
  %25 = insertvalue { ptr, i64 } undef, ptr %22, 0
  %26 = insertvalue { ptr, i64 } %25, i64 %24, 1
  ret { ptr, i64 } %26

bb4:                                              ; preds = %bb3
  %27 = getelementptr inbounds { { ptr, i64 }, { ptr, i64 } }, ptr %_2, i32 0, i32 1
  %28 = getelementptr inbounds { ptr, i64 }, ptr %27, i32 0, i32 0
  %_24.0 = load ptr, ptr %28, align 8, !nonnull !1, !align !3, !noundef !1
  %29 = getelementptr inbounds { ptr, i64 }, ptr %27, i32 0, i32 1
  %_24.1 = load i64, ptr %29, align 8
  %_10 = icmp eq i64 %_24.1, 0
  br i1 %_10, label %bb6, label %bb2

bb6:                                              ; preds = %bb4
  %30 = getelementptr inbounds { ptr, i64 }, ptr %_2, i32 0, i32 0
  %_25.0 = load ptr, ptr %30, align 8, !nonnull !1, !align !3, !noundef !1
  %31 = getelementptr inbounds { ptr, i64 }, ptr %_2, i32 0, i32 1
  %_25.1 = load i64, ptr %31, align 8
  %s = getelementptr inbounds [0 x { ptr, i64 }], ptr %_25.0, i64 0, i64 0
  %32 = getelementptr inbounds { ptr, i64 }, ptr %s, i32 0, i32 0
  %_26.0 = load ptr, ptr %32, align 8, !nonnull !1, !align !2, !noundef !1
  %33 = getelementptr inbounds { ptr, i64 }, ptr %s, i32 0, i32 1
  %_26.1 = load i64, ptr %33, align 8
  %34 = getelementptr inbounds { ptr, i64 }, ptr %0, i32 0, i32 0
  store ptr %_26.0, ptr %34, align 8
  %35 = getelementptr inbounds { ptr, i64 }, ptr %0, i32 0, i32 1
  store i64 %_26.1, ptr %35, align 8
  br label %bb7
}

; core::fmt::Arguments::new_v1
; Function Attrs: inlinehint uwtable
define internal void @_ZN4core3fmt9Arguments6new_v117hb7b30249326c03beE(ptr sret(%"core::fmt::Arguments<'_>") %0, ptr align 8 %pieces.0, i64 %pieces.1, ptr align 8 %args.0, i64 %args.1) unnamed_addr #0 {
start:
  %_24 = alloca { ptr, i64 }, align 8
  %_16 = alloca %"core::fmt::Arguments<'_>", align 8
  %_3 = alloca i8, align 1
  %_4 = icmp ult i64 %pieces.1, %args.1
  br i1 %_4, label %bb1, label %bb2

bb2:                                              ; preds = %start
  %_12 = add i64 %args.1, 1
  %_9 = icmp ugt i64 %pieces.1, %_12
  %1 = zext i1 %_9 to i8
  store i8 %1, ptr %_3, align 1
  br label %bb3

bb1:                                              ; preds = %start
  store i8 1, ptr %_3, align 1
  br label %bb3

bb3:                                              ; preds = %bb2, %bb1
  %2 = load i8, ptr %_3, align 1, !range !4, !noundef !1
  %3 = trunc i8 %2 to i1
  br i1 %3, label %bb4, label %bb6

bb6:                                              ; preds = %bb3
  store ptr null, ptr %_24, align 8
  %4 = getelementptr inbounds %"core::fmt::Arguments<'_>", ptr %0, i32 0, i32 1
  %5 = getelementptr inbounds { ptr, i64 }, ptr %4, i32 0, i32 0
  store ptr %pieces.0, ptr %5, align 8
  %6 = getelementptr inbounds { ptr, i64 }, ptr %4, i32 0, i32 1
  store i64 %pieces.1, ptr %6, align 8
  %7 = getelementptr inbounds { ptr, i64 }, ptr %_24, i32 0, i32 0
  %8 = load ptr, ptr %7, align 8, !align !3
  %9 = getelementptr inbounds { ptr, i64 }, ptr %_24, i32 0, i32 1
  %10 = load i64, ptr %9, align 8
  %11 = getelementptr inbounds { ptr, i64 }, ptr %0, i32 0, i32 0
  store ptr %8, ptr %11, align 8
  %12 = getelementptr inbounds { ptr, i64 }, ptr %0, i32 0, i32 1
  store i64 %10, ptr %12, align 8
  %13 = getelementptr inbounds %"core::fmt::Arguments<'_>", ptr %0, i32 0, i32 2
  %14 = getelementptr inbounds { ptr, i64 }, ptr %13, i32 0, i32 0
  store ptr %args.0, ptr %14, align 8
  %15 = getelementptr inbounds { ptr, i64 }, ptr %13, i32 0, i32 1
  store i64 %args.1, ptr %15, align 8
  ret void

bb4:                                              ; preds = %bb3
; call core::fmt::Arguments::new_v1
  call void @_ZN4core3fmt9Arguments6new_v117hb7b30249326c03beE(ptr sret(%"core::fmt::Arguments<'_>") %_16, ptr align 8 @alloc10, i64 1, ptr align 8 @alloc3, i64 0)
; call core::panicking::panic_fmt
  call void @_ZN4core9panicking9panic_fmt17h3678e6ca7972c44bE(ptr %_16, ptr align 8 @alloc15) #5
  unreachable
}

; core::ops::function::FnOnce::call_once
; Function Attrs: inlinehint uwtable
define internal void @_ZN4core3ops8function6FnOnce9call_once17hfcbf4c5d129a7570E(ptr sret(%"alloc::string::String") %0, ptr align 1 %1, i64 %2) unnamed_addr #0 {
start:
  %_2 = alloca { ptr, i64 }, align 8
  %3 = getelementptr inbounds { ptr, i64 }, ptr %_2, i32 0, i32 0
  store ptr %1, ptr %3, align 8
  %4 = getelementptr inbounds { ptr, i64 }, ptr %_2, i32 0, i32 1
  store i64 %2, ptr %4, align 8
  %5 = getelementptr inbounds { ptr, i64 }, ptr %_2, i32 0, i32 0
  %6 = load ptr, ptr %5, align 8, !nonnull !1, !align !2, !noundef !1
  %7 = getelementptr inbounds { ptr, i64 }, ptr %_2, i32 0, i32 1
  %8 = load i64, ptr %7, align 8
; call alloc::str::<impl alloc::borrow::ToOwned for str>::to_owned
  call void @"_ZN5alloc3str56_$LT$impl$u20$alloc..borrow..ToOwned$u20$for$u20$str$GT$8to_owned17ha5488b236e6f795fE"(ptr sret(%"alloc::string::String") %0, ptr align 1 %6, i64 %8)
  ret void
}

; core::ptr::drop_in_place<alloc::string::String>
; Function Attrs: uwtable
define void @"_ZN4core3ptr42drop_in_place$LT$alloc..string..String$GT$17hd78f5be0f21cdd27E"(ptr %_1) unnamed_addr #1 {
start:
; call core::ptr::drop_in_place<alloc::vec::Vec<u8>>
  call void @"_ZN4core3ptr46drop_in_place$LT$alloc..vec..Vec$LT$u8$GT$$GT$17hfcf0ebc264ee753eE"(ptr %_1)
  ret void
}

; core::ptr::drop_in_place<alloc::vec::Vec<u8>>
; Function Attrs: uwtable
define void @"_ZN4core3ptr46drop_in_place$LT$alloc..vec..Vec$LT$u8$GT$$GT$17hfcf0ebc264ee753eE"(ptr %_1) unnamed_addr #1 personality ptr @rust_eh_personality {
start:
  %0 = alloca { ptr, i32 }, align 8
; invoke <alloc::vec::Vec<T,A> as core::ops::drop::Drop>::drop
  invoke void @"_ZN70_$LT$alloc..vec..Vec$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h079fe8a7d96baf6fE"(ptr align 8 %_1)
          to label %bb4 unwind label %cleanup

bb3:                                              ; preds = %cleanup
; invoke core::ptr::drop_in_place<alloc::raw_vec::RawVec<u8>>
  invoke void @"_ZN4core3ptr53drop_in_place$LT$alloc..raw_vec..RawVec$LT$u8$GT$$GT$17h627423a3f0af0535E"(ptr %_1) #6
          to label %bb1 unwind label %abort

cleanup:                                          ; preds = %start
  %1 = landingpad { ptr, i32 }
          cleanup
  %2 = extractvalue { ptr, i32 } %1, 0
  %3 = extractvalue { ptr, i32 } %1, 1
  %4 = getelementptr inbounds { ptr, i32 }, ptr %0, i32 0, i32 0
  store ptr %2, ptr %4, align 8
  %5 = getelementptr inbounds { ptr, i32 }, ptr %0, i32 0, i32 1
  store i32 %3, ptr %5, align 8
  br label %bb3

bb4:                                              ; preds = %start
; call core::ptr::drop_in_place<alloc::raw_vec::RawVec<u8>>
  call void @"_ZN4core3ptr53drop_in_place$LT$alloc..raw_vec..RawVec$LT$u8$GT$$GT$17h627423a3f0af0535E"(ptr %_1)
  ret void

abort:                                            ; preds = %bb3
  %6 = landingpad { ptr, i32 }
          cleanup
  %7 = extractvalue { ptr, i32 } %6, 0
  %8 = extractvalue { ptr, i32 } %6, 1
; call core::panicking::panic_no_unwind
  call void @_ZN4core9panicking15panic_no_unwind17h3cb18ac821e7190cE() #7
  unreachable

bb1:                                              ; preds = %bb3
  %9 = load ptr, ptr %0, align 8
  %10 = getelementptr inbounds { ptr, i32 }, ptr %0, i32 0, i32 1
  %11 = load i32, ptr %10, align 8
  %12 = insertvalue { ptr, i32 } undef, ptr %9, 0
  %13 = insertvalue { ptr, i32 } %12, i32 %11, 1
  resume { ptr, i32 } %13
}

; core::ptr::drop_in_place<alloc::raw_vec::RawVec<u8>>
; Function Attrs: uwtable
define void @"_ZN4core3ptr53drop_in_place$LT$alloc..raw_vec..RawVec$LT$u8$GT$$GT$17h627423a3f0af0535E"(ptr %_1) unnamed_addr #1 {
start:
; call <alloc::raw_vec::RawVec<T,A> as core::ops::drop::Drop>::drop
  call void @"_ZN77_$LT$alloc..raw_vec..RawVec$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h1b215347b9a3f097E"(ptr align 8 %_1)
  ret void
}

; core::option::Option<T>::map_or_else
; Function Attrs: inlinehint uwtable
define void @"_ZN4core6option15Option$LT$T$GT$11map_or_else17h522f6970aa2216a9E"(ptr sret(%"alloc::string::String") %0, ptr align 1 %1, i64 %2, ptr align 8 %default) unnamed_addr #0 {
start:
  %_12 = alloca i8, align 1
  %_11 = alloca i8, align 1
  %_7 = alloca { ptr, i64 }, align 8
  %self = alloca { ptr, i64 }, align 8
  %3 = getelementptr inbounds { ptr, i64 }, ptr %self, i32 0, i32 0
  store ptr %1, ptr %3, align 8
  %4 = getelementptr inbounds { ptr, i64 }, ptr %self, i32 0, i32 1
  store i64 %2, ptr %4, align 8
  store i8 1, ptr %_12, align 1
  store i8 1, ptr %_11, align 1
  %5 = load ptr, ptr %self, align 8
  %6 = ptrtoint ptr %5 to i64
  %7 = icmp eq i64 %6, 0
  %_4 = select i1 %7, i64 0, i64 1
  %8 = icmp eq i64 %_4, 0
  br i1 %8, label %bb1, label %bb3

bb1:                                              ; preds = %start
  store i8 0, ptr %_12, align 1
; call alloc::fmt::format::{{closure}}
  call void @"_ZN5alloc3fmt6format28_$u7b$$u7b$closure$u7d$$u7d$17h9d295cb390b0d25cE"(ptr sret(%"alloc::string::String") %0, ptr align 8 %default)
  br label %bb9

bb3:                                              ; preds = %start
  %9 = getelementptr inbounds { ptr, i64 }, ptr %self, i32 0, i32 0
  %t.0 = load ptr, ptr %9, align 8, !nonnull !1, !align !2, !noundef !1
  %10 = getelementptr inbounds { ptr, i64 }, ptr %self, i32 0, i32 1
  %t.1 = load i64, ptr %10, align 8
  store i8 0, ptr %_11, align 1
  %11 = getelementptr inbounds { ptr, i64 }, ptr %_7, i32 0, i32 0
  store ptr %t.0, ptr %11, align 8
  %12 = getelementptr inbounds { ptr, i64 }, ptr %_7, i32 0, i32 1
  store i64 %t.1, ptr %12, align 8
  %13 = getelementptr inbounds { ptr, i64 }, ptr %_7, i32 0, i32 0
  %14 = load ptr, ptr %13, align 8, !nonnull !1, !align !2, !noundef !1
  %15 = getelementptr inbounds { ptr, i64 }, ptr %_7, i32 0, i32 1
  %16 = load i64, ptr %15, align 8
; call core::ops::function::FnOnce::call_once
  call void @_ZN4core3ops8function6FnOnce9call_once17hfcbf4c5d129a7570E(ptr sret(%"alloc::string::String") %0, ptr align 1 %14, i64 %16)
  br label %bb9

bb2:                                              ; No predecessors!
  unreachable

bb9:                                              ; preds = %bb1, %bb3
  %17 = load i8, ptr %_11, align 1, !range !4, !noundef !1
  %18 = trunc i8 %17 to i1
  br i1 %18, label %bb8, label %bb6

bb6:                                              ; preds = %bb8, %bb9
  %19 = load i8, ptr %_12, align 1, !range !4, !noundef !1
  %20 = trunc i8 %19 to i1
  br i1 %20, label %bb10, label %bb7

bb8:                                              ; preds = %bb9
  br label %bb6

bb7:                                              ; preds = %bb10, %bb6
  ret void

bb10:                                             ; preds = %bb6
  br label %bb7
}

; alloc::fmt::format
; Function Attrs: inlinehint uwtable
define internal void @_ZN5alloc3fmt6format17h2d80cb1cca362226E(ptr sret(%"alloc::string::String") %0, ptr %args) unnamed_addr #0 {
start:
  %_4 = alloca ptr, align 8
; call core::fmt::Arguments::as_str
  %1 = call { ptr, i64 } @_ZN4core3fmt9Arguments6as_str17hfbc55123107933a9E(ptr align 8 %args)
  %_2.0 = extractvalue { ptr, i64 } %1, 0
  %_2.1 = extractvalue { ptr, i64 } %1, 1
  store ptr %args, ptr %_4, align 8
  %2 = load ptr, ptr %_4, align 8, !nonnull !1, !align !3, !noundef !1
; call core::option::Option<T>::map_or_else
  call void @"_ZN4core6option15Option$LT$T$GT$11map_or_else17h522f6970aa2216a9E"(ptr sret(%"alloc::string::String") %0, ptr align 1 %_2.0, i64 %_2.1, ptr align 8 %2)
  ret void
}

; alloc::fmt::format::{{closure}}
; Function Attrs: inlinehint uwtable
define void @"_ZN5alloc3fmt6format28_$u7b$$u7b$closure$u7d$$u7d$17h9d295cb390b0d25cE"(ptr sret(%"alloc::string::String") %0, ptr align 8 %1) unnamed_addr #0 {
start:
  %_2 = alloca %"core::fmt::Arguments<'_>", align 8
  %_1 = alloca ptr, align 8
  store ptr %1, ptr %_1, align 8
  %_3 = load ptr, ptr %_1, align 8, !nonnull !1, !align !3, !noundef !1
  call void @llvm.memcpy.p0.p0.i64(ptr align 8 %_2, ptr align 8 %_3, i64 48, i1 false)
; call alloc::fmt::format::format_inner
  call void @_ZN5alloc3fmt6format12format_inner17h2c4b7c07ab802699E(ptr sret(%"alloc::string::String") %0, ptr %_2)
  ret void
}

; alloc::str::<impl alloc::borrow::ToOwned for str>::to_owned
; Function Attrs: inlinehint uwtable
define internal void @"_ZN5alloc3str56_$LT$impl$u20$alloc..borrow..ToOwned$u20$for$u20$str$GT$8to_owned17ha5488b236e6f795fE"(ptr sret(%"alloc::string::String") %0, ptr align 1 %self.0, i64 %self.1) unnamed_addr #0 {
start:
  %1 = alloca { ptr, i64 }, align 8
  %_2 = alloca %"alloc::vec::Vec<u8>", align 8
  store ptr %self.0, ptr %1, align 8
  %2 = getelementptr inbounds { ptr, i64 }, ptr %1, i32 0, i32 1
  store i64 %self.1, ptr %2, align 8
  %3 = load ptr, ptr %1, align 8, !nonnull !1, !align !2, !noundef !1
  %4 = getelementptr inbounds { ptr, i64 }, ptr %1, i32 0, i32 1
  %5 = load i64, ptr %4, align 8
  %6 = insertvalue { ptr, i64 } undef, ptr %3, 0
  %7 = insertvalue { ptr, i64 } %6, i64 %5, 1
  %_4.0 = extractvalue { ptr, i64 } %7, 0
  %_4.1 = extractvalue { ptr, i64 } %7, 1
; call alloc::slice::<impl alloc::borrow::ToOwned for [T]>::to_owned
  call void @"_ZN5alloc5slice64_$LT$impl$u20$alloc..borrow..ToOwned$u20$for$u20$$u5b$T$u5d$$GT$8to_owned17h80c1d8ab46a77049E"(ptr sret(%"alloc::vec::Vec<u8>") %_2, ptr align 1 %_4.0, i64 %_4.1)
; call alloc::string::String::from_utf8_unchecked
  call void @_ZN5alloc6string6String19from_utf8_unchecked17h69b05a65b58f06b1E(ptr sret(%"alloc::string::String") %0, ptr %_2)
  ret void
}

; alloc::string::String::from_utf8_unchecked
; Function Attrs: inlinehint uwtable
define internal void @_ZN5alloc6string6String19from_utf8_unchecked17h69b05a65b58f06b1E(ptr sret(%"alloc::string::String") %0, ptr %bytes) unnamed_addr #0 {
start:
  %_2 = alloca %"alloc::vec::Vec<u8>", align 8
  call void @llvm.memcpy.p0.p0.i64(ptr align 8 %_2, ptr align 8 %bytes, i64 24, i1 false)
  call void @llvm.memcpy.p0.p0.i64(ptr align 8 %0, ptr align 8 %_2, i64 24, i1 false)
  ret void
}

; probe1::probe
; Function Attrs: uwtable
define void @_ZN6probe15probe17hae56b8f5a6f7c516E() unnamed_addr #1 {
start:
  %_10 = alloca [1 x { ptr, ptr }], align 8
  %_3 = alloca %"core::fmt::Arguments<'_>", align 8
  %res = alloca %"alloc::string::String", align 8
  %_1 = alloca %"alloc::string::String", align 8
; call core::fmt::ArgumentV1::new_lower_exp
  %0 = call { ptr, ptr } @_ZN4core3fmt10ArgumentV113new_lower_exp17h599197eddb1cfb93E(ptr align 8 @alloc6)
  %_11.0 = extractvalue { ptr, ptr } %0, 0
  %_11.1 = extractvalue { ptr, ptr } %0, 1
  %1 = getelementptr inbounds [1 x { ptr, ptr }], ptr %_10, i64 0, i64 0
  %2 = getelementptr inbounds { ptr, ptr }, ptr %1, i32 0, i32 0
  store ptr %_11.0, ptr %2, align 8
  %3 = getelementptr inbounds { ptr, ptr }, ptr %1, i32 0, i32 1
  store ptr %_11.1, ptr %3, align 8
; call core::fmt::Arguments::new_v1
  call void @_ZN4core3fmt9Arguments6new_v117hb7b30249326c03beE(ptr sret(%"core::fmt::Arguments<'_>") %_3, ptr align 8 @alloc4, i64 1, ptr align 8 %_10, i64 1)
; call alloc::fmt::format
  call void @_ZN5alloc3fmt6format17h2d80cb1cca362226E(ptr sret(%"alloc::string::String") %res, ptr %_3)
  call void @llvm.memcpy.p0.p0.i64(ptr align 8 %_1, ptr align 8 %res, i64 24, i1 false)
; call core::ptr::drop_in_place<alloc::string::String>
  call void @"_ZN4core3ptr42drop_in_place$LT$alloc..string..String$GT$17hd78f5be0f21cdd27E"(ptr %_1)
  ret void
}

; core::fmt::num::imp::<impl core::fmt::LowerExp for isize>::fmt
; Function Attrs: uwtable
declare zeroext i1 @"_ZN4core3fmt3num3imp55_$LT$impl$u20$core..fmt..LowerExp$u20$for$u20$isize$GT$3fmt17h5876178a0bb85ab6E"(ptr align 8, ptr align 8) unnamed_addr #1

; core::panicking::panic_fmt
; Function Attrs: cold noinline noreturn uwtable
declare void @_ZN4core9panicking9panic_fmt17h3678e6ca7972c44bE(ptr, ptr align 8) unnamed_addr #2

; Function Attrs: uwtable
declare i32 @rust_eh_personality(i32, i32, i64, ptr, ptr) unnamed_addr #1

; <alloc::vec::Vec<T,A> as core::ops::drop::Drop>::drop
; Function Attrs: uwtable
declare void @"_ZN70_$LT$alloc..vec..Vec$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h079fe8a7d96baf6fE"(ptr align 8) unnamed_addr #1

; core::panicking::panic_no_unwind
; Function Attrs: cold noinline noreturn nounwind uwtable
declare void @_ZN4core9panicking15panic_no_unwind17h3cb18ac821e7190cE() unnamed_addr #3

; <alloc::raw_vec::RawVec<T,A> as core::ops::drop::Drop>::drop
; Function Attrs: uwtable
declare void @"_ZN77_$LT$alloc..raw_vec..RawVec$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h1b215347b9a3f097E"(ptr align 8) unnamed_addr #1

; Function Attrs: argmemonly nocallback nofree nounwind willreturn
declare void @llvm.memcpy.p0.p0.i64(ptr noalias nocapture writeonly, ptr noalias nocapture readonly, i64, i1 immarg) #4

; alloc::fmt::format::format_inner
; Function Attrs: uwtable
declare void @_ZN5alloc3fmt6format12format_inner17h2c4b7c07ab802699E(ptr sret(%"alloc::string::String"), ptr) unnamed_addr #1

; alloc::slice::<impl alloc::borrow::ToOwned for [T]>::to_owned
; Function Attrs: uwtable
declare void @"_ZN5alloc5slice64_$LT$impl$u20$alloc..borrow..ToOwned$u20$for$u20$$u5b$T$u5d$$GT$8to_owned17h80c1d8ab46a77049E"(ptr sret(%"alloc::vec::Vec<u8>"), ptr align 1, i64) unnamed_addr #1

attributes #0 = { inlinehint uwtable "frame-pointer"="non-leaf" "target-cpu"="apple-a14" "target-features"="+v8a" }
attributes #1 = { uwtable "frame-pointer"="non-leaf" "target-cpu"="apple-a14" "target-features"="+v8a" }
attributes #2 = { cold noinline noreturn uwtable "frame-pointer"="non-leaf" "target-cpu"="apple-a14" "target-features"="+v8a" }
attributes #3 = { cold noinline noreturn nounwind uwtable "frame-pointer"="non-leaf" "target-cpu"="apple-a14" "target-features"="+v8a" }
attributes #4 = { argmemonly nocallback nofree nounwind willreturn }
attributes #5 = { noreturn }
attributes #6 = { noinline }
attributes #7 = { noinline noreturn nounwind }

!llvm.module.flags = !{!0}

!0 = !{i32 7, !"PIC Level", i32 2}
!1 = !{}
!2 = !{i64 1}
!3 = !{i64 8}
!4 = !{i8 0, i8 2}
