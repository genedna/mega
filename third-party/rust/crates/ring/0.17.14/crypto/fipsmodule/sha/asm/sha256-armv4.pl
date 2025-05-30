#! /usr/bin/env perl
# Copyright 2007-2016 The OpenSSL Project Authors. All Rights Reserved.
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#     https://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.


# ====================================================================
# Written by Andy Polyakov <appro@openssl.org> for the OpenSSL
# project.
# ====================================================================

# SHA256 block procedure for ARMv4. May 2007.

# Performance is ~2x better than gcc 3.4 generated code and in "abso-
# lute" terms is ~2250 cycles per 64-byte block or ~35 cycles per
# byte [on single-issue Xscale PXA250 core].

# July 2010.
#
# Rescheduling for dual-issue pipeline resulted in 22% improvement on
# Cortex A8 core and ~20 cycles per processed byte.

# February 2011.
#
# Profiler-assisted and platform-specific optimization resulted in 16%
# improvement on Cortex A8 core and ~15.4 cycles per processed byte.

# September 2013.
#
# Add NEON implementation. On Cortex A8 it was measured to process one
# byte in 12.5 cycles or 23% faster than integer-only code. Snapdragon
# S4 does it in 12.5 cycles too, but it's 50% faster than integer-only
# code (meaning that latter performs sub-optimally, nothing was done
# about it).

# May 2014.
#
# Add ARMv8 code path performing at 2.0 cpb on Apple A7.

$flavour = shift;
if ($flavour=~/\w[\w\-]*\.\w+$/) { $output=$flavour; undef $flavour; }
else { while (($output=shift) && ($output!~/\w[\w\-]*\.\w+$/)) {} }

if ($flavour && $flavour ne "void") {
    $0 =~ m/(.*[\/\\])[^\/\\]+$/; $dir=$1;
    ( $xlate="${dir}arm-xlate.pl" and -f $xlate ) or
    ( $xlate="${dir}../../../perlasm/arm-xlate.pl" and -f $xlate) or
    die "can't locate arm-xlate.pl";

    open OUT,"| \"$^X\" \"$xlate\" $flavour \"$output\"";
    *STDOUT=*OUT;
} else {
    open OUT,">$output";
    *STDOUT=*OUT;
}

$ctx="r0";	$t0="r0";
$inp="r1";	$t4="r1";
$len="r2";	$t1="r2";
$T1="r3";	$t3="r3";
$A="r4";
$B="r5";
$C="r6";
$D="r7";
$E="r8";
$F="r9";
$G="r10";
$H="r11";
@V=($A,$B,$C,$D,$E,$F,$G,$H);
$t2="r12";
$Ktbl="r14";

@Sigma0=( 2,13,22);
@Sigma1=( 6,11,25);
@sigma0=( 7,18, 3);
@sigma1=(17,19,10);

sub BODY_00_15 {
my ($i,$a,$b,$c,$d,$e,$f,$g,$h) = @_;

$code.=<<___ if ($i<16);
#if __ARM_ARCH>=7
	@ ldr	$t1,[$inp],#4			@ $i
# if $i==15
	str	$inp,[sp,#17*4]			@ make room for $t4
# endif
	eor	$t0,$e,$e,ror#`$Sigma1[1]-$Sigma1[0]`
	add	$a,$a,$t2			@ h+=Maj(a,b,c) from the past
	eor	$t0,$t0,$e,ror#`$Sigma1[2]-$Sigma1[0]`	@ Sigma1(e)
# ifndef __ARMEB__
	rev	$t1,$t1
# endif
#else
	@ ldrb	$t1,[$inp,#3]			@ $i
	add	$a,$a,$t2			@ h+=Maj(a,b,c) from the past
	ldrb	$t2,[$inp,#2]
	ldrb	$t0,[$inp,#1]
	orr	$t1,$t1,$t2,lsl#8
	ldrb	$t2,[$inp],#4
	orr	$t1,$t1,$t0,lsl#16
# if $i==15
	str	$inp,[sp,#17*4]			@ make room for $t4
# endif
	eor	$t0,$e,$e,ror#`$Sigma1[1]-$Sigma1[0]`
	orr	$t1,$t1,$t2,lsl#24
	eor	$t0,$t0,$e,ror#`$Sigma1[2]-$Sigma1[0]`	@ Sigma1(e)
#endif
___
$code.=<<___;
	ldr	$t2,[$Ktbl],#4			@ *K256++
	add	$h,$h,$t1			@ h+=X[i]
	str	$t1,[sp,#`$i%16`*4]
	eor	$t1,$f,$g
	add	$h,$h,$t0,ror#$Sigma1[0]	@ h+=Sigma1(e)
	and	$t1,$t1,$e
	add	$h,$h,$t2			@ h+=K256[i]
	eor	$t1,$t1,$g			@ Ch(e,f,g)
	eor	$t0,$a,$a,ror#`$Sigma0[1]-$Sigma0[0]`
	add	$h,$h,$t1			@ h+=Ch(e,f,g)
#if $i==31
	and	$t2,$t2,#0xff
	cmp	$t2,#0xf2			@ done?
#endif
#if $i<15
# if __ARM_ARCH>=7
	ldr	$t1,[$inp],#4			@ prefetch
# else
	ldrb	$t1,[$inp,#3]
# endif
	eor	$t2,$a,$b			@ a^b, b^c in next round
#else
	ldr	$t1,[sp,#`($i+2)%16`*4]		@ from future BODY_16_xx
	eor	$t2,$a,$b			@ a^b, b^c in next round
	ldr	$t4,[sp,#`($i+15)%16`*4]	@ from future BODY_16_xx
#endif
	eor	$t0,$t0,$a,ror#`$Sigma0[2]-$Sigma0[0]`	@ Sigma0(a)
	and	$t3,$t3,$t2			@ (b^c)&=(a^b)
	add	$d,$d,$h			@ d+=h
	eor	$t3,$t3,$b			@ Maj(a,b,c)
	add	$h,$h,$t0,ror#$Sigma0[0]	@ h+=Sigma0(a)
	@ add	$h,$h,$t3			@ h+=Maj(a,b,c)
___
	($t2,$t3)=($t3,$t2);
}

sub BODY_16_XX {
my ($i,$a,$b,$c,$d,$e,$f,$g,$h) = @_;

$code.=<<___;
	@ ldr	$t1,[sp,#`($i+1)%16`*4]		@ $i
	@ ldr	$t4,[sp,#`($i+14)%16`*4]
	mov	$t0,$t1,ror#$sigma0[0]
	add	$a,$a,$t2			@ h+=Maj(a,b,c) from the past
	mov	$t2,$t4,ror#$sigma1[0]
	eor	$t0,$t0,$t1,ror#$sigma0[1]
	eor	$t2,$t2,$t4,ror#$sigma1[1]
	eor	$t0,$t0,$t1,lsr#$sigma0[2]	@ sigma0(X[i+1])
	ldr	$t1,[sp,#`($i+0)%16`*4]
	eor	$t2,$t2,$t4,lsr#$sigma1[2]	@ sigma1(X[i+14])
	ldr	$t4,[sp,#`($i+9)%16`*4]

	add	$t2,$t2,$t0
	eor	$t0,$e,$e,ror#`$Sigma1[1]-$Sigma1[0]`	@ from BODY_00_15
	add	$t1,$t1,$t2
	eor	$t0,$t0,$e,ror#`$Sigma1[2]-$Sigma1[0]`	@ Sigma1(e)
	add	$t1,$t1,$t4			@ X[i]
___
	&BODY_00_15(@_);
}

$code=<<___;
#ifdef __KERNEL__
# define __ARM_ARCH __LINUX_ARM_ARCH__
# define __ARM_MAX_ARCH__ 7
#endif

@ Silence ARMv8 deprecated IT instruction warnings. This file is used by both
@ ARMv7 and ARMv8 processors. It does have ARMv8-only code, but those
@ instructions are manually-encoded. (See unsha256.)
.arch  armv7-a

.text
#if defined(__thumb2__)
.syntax unified
.thumb
#else
.code   32
#endif

.type	K256,%object
.align	5
K256:
.word	0x428a2f98,0x71374491,0xb5c0fbcf,0xe9b5dba5
.word	0x3956c25b,0x59f111f1,0x923f82a4,0xab1c5ed5
.word	0xd807aa98,0x12835b01,0x243185be,0x550c7dc3
.word	0x72be5d74,0x80deb1fe,0x9bdc06a7,0xc19bf174
.word	0xe49b69c1,0xefbe4786,0x0fc19dc6,0x240ca1cc
.word	0x2de92c6f,0x4a7484aa,0x5cb0a9dc,0x76f988da
.word	0x983e5152,0xa831c66d,0xb00327c8,0xbf597fc7
.word	0xc6e00bf3,0xd5a79147,0x06ca6351,0x14292967
.word	0x27b70a85,0x2e1b2138,0x4d2c6dfc,0x53380d13
.word	0x650a7354,0x766a0abb,0x81c2c92e,0x92722c85
.word	0xa2bfe8a1,0xa81a664b,0xc24b8b70,0xc76c51a3
.word	0xd192e819,0xd6990624,0xf40e3585,0x106aa070
.word	0x19a4c116,0x1e376c08,0x2748774c,0x34b0bcb5
.word	0x391c0cb3,0x4ed8aa4a,0x5b9cca4f,0x682e6ff3
.word	0x748f82ee,0x78a5636f,0x84c87814,0x8cc70208
.word	0x90befffa,0xa4506ceb,0xbef9a3f7,0xc67178f2
.size	K256,.-K256
.word	0				@ terminator
.align	5

.global	sha256_block_data_order_nohw
.type	sha256_block_data_order_nohw,%function
sha256_block_data_order_nohw:
	add	$len,$inp,$len,lsl#6	@ len to point at the end of inp
	stmdb	sp!,{$ctx,$inp,$len,r4-r11,lr}
	ldmia	$ctx,{$A,$B,$C,$D,$E,$F,$G,$H}
	adr	$Ktbl,K256
	sub	sp,sp,#16*4		@ alloca(X[16])
.Loop:
# if __ARM_ARCH>=7
	ldr	$t1,[$inp],#4
# else
	ldrb	$t1,[$inp,#3]
# endif
	eor	$t3,$B,$C		@ magic
	eor	$t2,$t2,$t2
___
for($i=0;$i<16;$i++)	{ &BODY_00_15($i,@V); unshift(@V,pop(@V)); }
$code.=".Lrounds_16_xx:\n";
for (;$i<32;$i++)	{ &BODY_16_XX($i,@V); unshift(@V,pop(@V)); }
$code.=<<___;
#if __ARM_ARCH>=7
	ite	eq			@ Thumb2 thing, sanity check in ARM
#endif
	ldreq	$t3,[sp,#16*4]		@ pull ctx
	bne	.Lrounds_16_xx

	add	$A,$A,$t2		@ h+=Maj(a,b,c) from the past
	ldr	$t0,[$t3,#0]
	ldr	$t1,[$t3,#4]
	ldr	$t2,[$t3,#8]
	add	$A,$A,$t0
	ldr	$t0,[$t3,#12]
	add	$B,$B,$t1
	ldr	$t1,[$t3,#16]
	add	$C,$C,$t2
	ldr	$t2,[$t3,#20]
	add	$D,$D,$t0
	ldr	$t0,[$t3,#24]
	add	$E,$E,$t1
	ldr	$t1,[$t3,#28]
	add	$F,$F,$t2
	ldr	$inp,[sp,#17*4]		@ pull inp
	ldr	$t2,[sp,#18*4]		@ pull inp+len
	add	$G,$G,$t0
	add	$H,$H,$t1
	stmia	$t3,{$A,$B,$C,$D,$E,$F,$G,$H}
	cmp	$inp,$t2
	sub	$Ktbl,$Ktbl,#256	@ rewind Ktbl
	bne	.Loop

	add	sp,sp,#`16+3`*4	@ destroy frame
#if __ARM_ARCH>=5
	ldmia	sp!,{r4-r11,pc}
#else
	ldmia	sp!,{r4-r11,lr}
	tst	lr,#1
	moveq	pc,lr			@ be binary compatible with V4, yet
	bx	lr			@ interoperable with Thumb ISA:-)
#endif
.size	sha256_block_data_order_nohw,.-sha256_block_data_order_nohw
___
######################################################################
# NEON stuff
#
{{{
my @X=map("q$_",(0..3));
my ($T0,$T1,$T2,$T3,$T4,$T5)=("q8","q9","q10","q11","d24","d25");
my $Xfer=$t4;
my $j=0;

sub Dlo()   { shift=~m|q([1]?[0-9])|?"d".($1*2):"";     }
sub Dhi()   { shift=~m|q([1]?[0-9])|?"d".($1*2+1):"";   }

sub AUTOLOAD()          # thunk [simplified] x86-style perlasm
{ my $opcode = $AUTOLOAD; $opcode =~ s/.*:://; $opcode =~ s/_/\./;
  my $arg = pop;
    $arg = "#$arg" if ($arg*1 eq $arg);
    $code .= "\t$opcode\t".join(',',@_,$arg)."\n";
}

sub Xupdate()
{ use integer;
  my $body = shift;
  my @insns = (&$body,&$body,&$body,&$body);
  my ($a,$b,$c,$d,$e,$f,$g,$h);

	&vext_8		($T0,@X[0],@X[1],4);	# X[1..4]
	 eval(shift(@insns));
	 eval(shift(@insns));
	 eval(shift(@insns));
	&vext_8		($T1,@X[2],@X[3],4);	# X[9..12]
	 eval(shift(@insns));
	 eval(shift(@insns));
	 eval(shift(@insns));
	&vshr_u32	($T2,$T0,$sigma0[0]);
	 eval(shift(@insns));
	 eval(shift(@insns));
	&vadd_i32	(@X[0],@X[0],$T1);	# X[0..3] += X[9..12]
	 eval(shift(@insns));
	 eval(shift(@insns));
	&vshr_u32	($T1,$T0,$sigma0[2]);
	 eval(shift(@insns));
	 eval(shift(@insns));
	&vsli_32	($T2,$T0,32-$sigma0[0]);
	 eval(shift(@insns));
	 eval(shift(@insns));
	&vshr_u32	($T3,$T0,$sigma0[1]);
	 eval(shift(@insns));
	 eval(shift(@insns));
	&veor		($T1,$T1,$T2);
	 eval(shift(@insns));
	 eval(shift(@insns));
	&vsli_32	($T3,$T0,32-$sigma0[1]);
	 eval(shift(@insns));
	 eval(shift(@insns));
	  &vshr_u32	($T4,&Dhi(@X[3]),$sigma1[0]);
	 eval(shift(@insns));
	 eval(shift(@insns));
	&veor		($T1,$T1,$T3);		# sigma0(X[1..4])
	 eval(shift(@insns));
	 eval(shift(@insns));
	  &vsli_32	($T4,&Dhi(@X[3]),32-$sigma1[0]);
	 eval(shift(@insns));
	 eval(shift(@insns));
	  &vshr_u32	($T5,&Dhi(@X[3]),$sigma1[2]);
	 eval(shift(@insns));
	 eval(shift(@insns));
	&vadd_i32	(@X[0],@X[0],$T1);	# X[0..3] += sigma0(X[1..4])
	 eval(shift(@insns));
	 eval(shift(@insns));
	  &veor		($T5,$T5,$T4);
	 eval(shift(@insns));
	 eval(shift(@insns));
	  &vshr_u32	($T4,&Dhi(@X[3]),$sigma1[1]);
	 eval(shift(@insns));
	 eval(shift(@insns));
	  &vsli_32	($T4,&Dhi(@X[3]),32-$sigma1[1]);
	 eval(shift(@insns));
	 eval(shift(@insns));
	  &veor		($T5,$T5,$T4);		# sigma1(X[14..15])
	 eval(shift(@insns));
	 eval(shift(@insns));
	&vadd_i32	(&Dlo(@X[0]),&Dlo(@X[0]),$T5);# X[0..1] += sigma1(X[14..15])
	 eval(shift(@insns));
	 eval(shift(@insns));
	  &vshr_u32	($T4,&Dlo(@X[0]),$sigma1[0]);
	 eval(shift(@insns));
	 eval(shift(@insns));
	  &vsli_32	($T4,&Dlo(@X[0]),32-$sigma1[0]);
	 eval(shift(@insns));
	 eval(shift(@insns));
	  &vshr_u32	($T5,&Dlo(@X[0]),$sigma1[2]);
	 eval(shift(@insns));
	 eval(shift(@insns));
	  &veor		($T5,$T5,$T4);
	 eval(shift(@insns));
	 eval(shift(@insns));
	  &vshr_u32	($T4,&Dlo(@X[0]),$sigma1[1]);
	 eval(shift(@insns));
	 eval(shift(@insns));
	&vld1_32	("{$T0}","[$Ktbl,:128]!");
	 eval(shift(@insns));
	 eval(shift(@insns));
	  &vsli_32	($T4,&Dlo(@X[0]),32-$sigma1[1]);
	 eval(shift(@insns));
	 eval(shift(@insns));
	  &veor		($T5,$T5,$T4);		# sigma1(X[16..17])
	 eval(shift(@insns));
	 eval(shift(@insns));
	&vadd_i32	(&Dhi(@X[0]),&Dhi(@X[0]),$T5);# X[2..3] += sigma1(X[16..17])
	 eval(shift(@insns));
	 eval(shift(@insns));
	&vadd_i32	($T0,$T0,@X[0]);
	 while($#insns>=2) { eval(shift(@insns)); }
	&vst1_32	("{$T0}","[$Xfer,:128]!");
	 eval(shift(@insns));
	 eval(shift(@insns));

	push(@X,shift(@X));		# "rotate" X[]
}

sub Xpreload()
{ use integer;
  my $body = shift;
  my @insns = (&$body,&$body,&$body,&$body);
  my ($a,$b,$c,$d,$e,$f,$g,$h);

	 eval(shift(@insns));
	 eval(shift(@insns));
	 eval(shift(@insns));
	 eval(shift(@insns));
	&vld1_32	("{$T0}","[$Ktbl,:128]!");
	 eval(shift(@insns));
	 eval(shift(@insns));
	 eval(shift(@insns));
	 eval(shift(@insns));
	&vrev32_8	(@X[0],@X[0]);
	 eval(shift(@insns));
	 eval(shift(@insns));
	 eval(shift(@insns));
	 eval(shift(@insns));
	&vadd_i32	($T0,$T0,@X[0]);
	 foreach (@insns) { eval; }	# remaining instructions
	&vst1_32	("{$T0}","[$Xfer,:128]!");

	push(@X,shift(@X));		# "rotate" X[]
}

sub body_00_15 () {
	(
	'($a,$b,$c,$d,$e,$f,$g,$h)=@V;'.
	'&add	($h,$h,$t1)',			# h+=X[i]+K[i]
	'&eor	($t1,$f,$g)',
	'&eor	($t0,$e,$e,"ror#".($Sigma1[1]-$Sigma1[0]))',
	'&add	($a,$a,$t2)',			# h+=Maj(a,b,c) from the past
	'&and	($t1,$t1,$e)',
	'&eor	($t2,$t0,$e,"ror#".($Sigma1[2]-$Sigma1[0]))',	# Sigma1(e)
	'&eor	($t0,$a,$a,"ror#".($Sigma0[1]-$Sigma0[0]))',
	'&eor	($t1,$t1,$g)',			# Ch(e,f,g)
	'&add	($h,$h,$t2,"ror#$Sigma1[0]")',	# h+=Sigma1(e)
	'&eor	($t2,$a,$b)',			# a^b, b^c in next round
	'&eor	($t0,$t0,$a,"ror#".($Sigma0[2]-$Sigma0[0]))',	# Sigma0(a)
	'&add	($h,$h,$t1)',			# h+=Ch(e,f,g)
	'&ldr	($t1,sprintf "[sp,#%d]",4*(($j+1)&15))	if (($j&15)!=15);'.
	'&ldr	($t1,"[$Ktbl]")				if ($j==15);'.
	'&ldr	($t1,"[sp,#64]")			if ($j==31)',
	'&and	($t3,$t3,$t2)',			# (b^c)&=(a^b)
	'&add	($d,$d,$h)',			# d+=h
	'&add	($h,$h,$t0,"ror#$Sigma0[0]");'.	# h+=Sigma0(a)
	'&eor	($t3,$t3,$b)',			# Maj(a,b,c)
	'$j++;	unshift(@V,pop(@V)); ($t2,$t3)=($t3,$t2);'
	)
}

$code.=<<___;
#if __ARM_MAX_ARCH__>=7
.arch	armv7-a
.fpu	neon

.LK256_shortcut_neon:
@ PC is 8 bytes ahead in Arm mode and 4 bytes ahead in Thumb mode.
#if defined(__thumb2__)
.word	K256-(.LK256_add_neon+4)
#else
.word	K256-(.LK256_add_neon+8)
#endif

.global	sha256_block_data_order_neon
.type	sha256_block_data_order_neon,%function
.align	5
.skip	16
sha256_block_data_order_neon:
	stmdb	sp!,{r4-r12,lr}

	sub	$H,sp,#16*4+16

	@ K256 is just at the boundary of being easily referenced by an ADR from
	@ this function. In Arm mode, when building with __ARM_ARCH=6, it does
	@ not fit. By moving code around, we could make it fit, but this is too
	@ fragile. For simplicity, just load the offset from
	@ .LK256_shortcut_neon.
	@
	@ TODO(davidben): adrl would avoid a load, but clang-assembler does not
	@ support it. We might be able to emulate it with a macro, but Android's
	@ did not work when I tried it.
	@ https://android.googlesource.com/platform/ndk/+/refs/heads/main/docs/ClangMigration.md#arm
	ldr	$Ktbl,.LK256_shortcut_neon
.LK256_add_neon:
	add	$Ktbl,pc,$Ktbl

	bic	$H,$H,#15		@ align for 128-bit stores
	mov	$t2,sp
	mov	sp,$H			@ alloca
	add	$len,$inp,$len,lsl#6	@ len to point at the end of inp

	vld1.8		{@X[0]},[$inp]!
	vld1.8		{@X[1]},[$inp]!
	vld1.8		{@X[2]},[$inp]!
	vld1.8		{@X[3]},[$inp]!
	vld1.32		{$T0},[$Ktbl,:128]!
	vld1.32		{$T1},[$Ktbl,:128]!
	vld1.32		{$T2},[$Ktbl,:128]!
	vld1.32		{$T3},[$Ktbl,:128]!
	vrev32.8	@X[0],@X[0]		@ yes, even on
	str		$ctx,[sp,#64]
	vrev32.8	@X[1],@X[1]		@ big-endian
	str		$inp,[sp,#68]
	mov		$Xfer,sp
	vrev32.8	@X[2],@X[2]
	str		$len,[sp,#72]
	vrev32.8	@X[3],@X[3]
	str		$t2,[sp,#76]		@ save original sp
	vadd.i32	$T0,$T0,@X[0]
	vadd.i32	$T1,$T1,@X[1]
	vst1.32		{$T0},[$Xfer,:128]!
	vadd.i32	$T2,$T2,@X[2]
	vst1.32		{$T1},[$Xfer,:128]!
	vadd.i32	$T3,$T3,@X[3]
	vst1.32		{$T2},[$Xfer,:128]!
	vst1.32		{$T3},[$Xfer,:128]!

	ldmia		$ctx,{$A-$H}
	sub		$Xfer,$Xfer,#64
	ldr		$t1,[sp,#0]
	eor		$t2,$t2,$t2
	eor		$t3,$B,$C
	b		.L_00_48

.align	4
.L_00_48:
___
	&Xupdate(\&body_00_15);
	&Xupdate(\&body_00_15);
	&Xupdate(\&body_00_15);
	&Xupdate(\&body_00_15);
$code.=<<___;
	teq	$t1,#0				@ check for K256 terminator
	ldr	$t1,[sp,#0]
	sub	$Xfer,$Xfer,#64
	bne	.L_00_48

	ldr		$inp,[sp,#68]
	ldr		$t0,[sp,#72]
	sub		$Ktbl,$Ktbl,#256	@ rewind $Ktbl
	teq		$inp,$t0
	it		eq
	subeq		$inp,$inp,#64		@ avoid SEGV
	vld1.8		{@X[0]},[$inp]!		@ load next input block
	vld1.8		{@X[1]},[$inp]!
	vld1.8		{@X[2]},[$inp]!
	vld1.8		{@X[3]},[$inp]!
	it		ne
	strne		$inp,[sp,#68]
	mov		$Xfer,sp
___
	&Xpreload(\&body_00_15);
	&Xpreload(\&body_00_15);
	&Xpreload(\&body_00_15);
	&Xpreload(\&body_00_15);
$code.=<<___;
	ldr	$t0,[$t1,#0]
	add	$A,$A,$t2			@ h+=Maj(a,b,c) from the past
	ldr	$t2,[$t1,#4]
	ldr	$t3,[$t1,#8]
	ldr	$t4,[$t1,#12]
	add	$A,$A,$t0			@ accumulate
	ldr	$t0,[$t1,#16]
	add	$B,$B,$t2
	ldr	$t2,[$t1,#20]
	add	$C,$C,$t3
	ldr	$t3,[$t1,#24]
	add	$D,$D,$t4
	ldr	$t4,[$t1,#28]
	add	$E,$E,$t0
	str	$A,[$t1],#4
	add	$F,$F,$t2
	str	$B,[$t1],#4
	add	$G,$G,$t3
	str	$C,[$t1],#4
	add	$H,$H,$t4
	str	$D,[$t1],#4
	stmia	$t1,{$E-$H}

	ittte	ne
	movne	$Xfer,sp
	ldrne	$t1,[sp,#0]
	eorne	$t2,$t2,$t2
	ldreq	sp,[sp,#76]			@ restore original sp
	itt	ne
	eorne	$t3,$B,$C
	bne	.L_00_48

	ldmia	sp!,{r4-r12,pc}
.size	sha256_block_data_order_neon,.-sha256_block_data_order_neon
#endif
___
}}}

$code.=<<___;
.asciz  "SHA256 block transform for ARMv4/NEON/ARMv8, CRYPTOGAMS by <appro\@openssl.org>"
.align	2
___

open SELF,$0;
while(<SELF>) {
	next if (/^#!/);
	last if (!s/^#/@/ and !/^$/);
	print;
}
close SELF;

foreach (split($/,$code)) {

	s/\`([^\`]*)\`/eval $1/geo;

	s/\bret\b/bx	lr/go		or
	s/\bbx\s+lr\b/.word\t0xe12fff1e/go;	# make it possible to compile with -march=armv4

	print $_,"\n";
}

close STDOUT or die "error closing STDOUT: $!"; # enforce flush
