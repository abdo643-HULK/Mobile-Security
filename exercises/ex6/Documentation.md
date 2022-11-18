---
title: Exercise 6
author: Abd El Rahaman Shehata

header-includes:
	- \usepackage{fvextra}
	- \DefineVerbatimEnvironment{Highlighting}{Verbatim}{breaklines,commandchars=\\\{\}}
---

# Exercise 6

## 1. Reverse engineer the following Dalvik bytecode snippet into high-level Java code, and explain what the code does.

```smali
.method private mymethod([II)V
	aget v0, v3, v4
	add-int/lit8 v1, v4, 0x1
	aget v1, v3, v1
	aput v1, v3, v4
	add-int/lit8 v1, v4, 0x1
	aput v0, v3, v1
	return-void
.end method
```

Notes:

-   First argument passed to method is v3, and of type [I
-   Second argument passed to method is v4, and of type I

### Code Analysis

```smali
# takes as the first parameter an int array, int as the second one and returns void
.method private mymethod(
	# Int Array
	[I
	# Int
	I)V # Void return
	# Loads value into v0 from array in v3 at index in v4
	aget v0, v3, v4
	# Adds the value in v4 to a 0x1 (8bit literal) and stores it in v1
	add-int/lit8 v1, v4, 0x1
	# Loads value into v1 from array in v3 at index in v1
	aget v1, v3, v1
	# Stores value from v1 into array in v3 at index in v4
	aput v1, v3, v4
	# Adds the value in v4 to a 0x1 (8bit literal) and stores it in v1
	add-int/lit8 v1, v4, 0x1
	# Stores value from v0 into array in v3 at index in v1
	aput v0, v3, v1
	# Return from a void method
	return-void
.end method
```

### Java Code

```java
private void mymethod(int[] v3, int v4) {
	var v0 = v3[v4];
	var v1 = v4 + 0x1;
	v1 = v3[v1];
	v3[v4] = v1;
	v1 = v4 + 0x1;
	v3[v1] = v0;
}
```

### Explanation

The Method swaps the value at the given index with the next one.

## 2. Reverse engineer the following Dalvik bytecode snippet into high-level Java code, and explain what the code does. Note: The Java code does not need to be a compilable project, but must be syntactically valid Java

```smali
.method private mymethod2(Ljava/io/InputStream;)I
	# Exception handling definition
	.catch Ljava/io/IOException; {:try_start_0 .. :try_end_0} :handler_0

	# Start of try block
	:try_start_0
	# Calling a method stored in a vtable on the object passed in as a parameter
	invoke-virtual {v2},Ljava/io/InputStream/read
	# Saves the result of the method call in register v0
	move-result v0
	# End of try block
	:try_end_0
	# Return value in register v0
	return v0
	# Start of catch block
	:handler_0
	# Move Exception into register v0
	move-exception v0
	# Move 15 (4bits) into register v0
	const/4 v0,15
	goto :try_end_0
.end method
```

```java
private int mymethod2(java.io.InputStream v2) {
	int v0;
	try {
		v0 = v2.read();
	} catch (java.io.IOException e) {
		v0 = 15;
	}
	return v0;
}
```

## 3. Choose any Android application available in the Google Play Store (e.g. via apkpure.com), download and disassemble it. Analyze (and possibly modify) the SMALI bytecode in order to achieve a particular goal. If your goal requires code modification, re-package the APK and run it on an Android system. Interesting goals include, but are not limited to:

-   Removing ad banners
-   Transforming a ”free” to a ”paid” application (for research purposes only)
-   Bypass authentication,
-   Analyze code in order to understand authentication, credential storage, certificate checking, ...
-   ...or anything else interesting really

Submission: Describe your goal and the exact process what you did, what you found, what
problems you faced, how you got your findings, etc.
