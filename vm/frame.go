package vm

import (
	"monkey/code"
	"monkey/object"
)

type Frame struct {
	cl *object.Closure
	// instruction pointer in _this_ frame for _this_ function
	ip int
	// Points to the bottom of the stack of the curret call frame
	basePointer int
}

func NewFrame(cl *object.Closure, basePointer int) *Frame {
	return &Frame{cl: cl, ip: -1, basePointer: basePointer}
}

func (f *Frame) Instructions() code.Instructions {
	return f.cl.Fn.Instructions
}
