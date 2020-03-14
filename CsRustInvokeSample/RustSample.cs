using System;
using System.Collections.Generic;
using System.Linq;
using System.Runtime.InteropServices;
using System.Text;
using System.Threading.Tasks;

namespace CsRustInvokeSample
{
    class RustSample : IDisposable
    {
        [DllImport("cs_rust_invoke", EntryPoint = "create_rust_sample_instance")]
        static extern uint CreateRustSampleInstance(IntPtr[] buffer, uint bufferSize);

        delegate void FnAction(IntPtr self);
        delegate int FnGetValue(IntPtr self);
        delegate void FnCalc(IntPtr self, int value);
        delegate uint FnAppendChars(IntPtr self, [MarshalAs(UnmanagedType.LPUTF8Str)] string s);

        IntPtr _self;
        FnAction _fnDestroy;
        FnGetValue _fnGetCurrentValue;
        FnCalc _fnAdd;
        FnCalc _fnSub;
        FnAppendChars _fnAppendChars;
        FnAction _fnPrintChars;

        public RustSample()
        {
            uint bufferSize = CreateRustSampleInstance(null, 0);
            var buffer = new IntPtr[bufferSize];
            if (CreateRustSampleInstance(buffer, bufferSize) != bufferSize)
            {
                throw new Exception();
            }

            _self = buffer[0];
            _fnDestroy = Marshal.GetDelegateForFunctionPointer<FnAction>(buffer[1]);
            _fnGetCurrentValue = Marshal.GetDelegateForFunctionPointer<FnGetValue>(buffer[2]);
            _fnAdd = Marshal.GetDelegateForFunctionPointer<FnCalc>(buffer[3]);
            _fnSub = Marshal.GetDelegateForFunctionPointer<FnCalc>(buffer[4]);
            _fnAppendChars = Marshal.GetDelegateForFunctionPointer<FnAppendChars>(buffer[5]);
            _fnPrintChars = Marshal.GetDelegateForFunctionPointer<FnAction>(buffer[6]);
        }

        public void Dispose()
        {
            _fnDestroy?.Invoke(_self);
            _fnDestroy = null;
            _self = IntPtr.Zero;
        }

        public int GetCurrentValue() => _fnGetCurrentValue(_self);
        public void Add(int value) => _fnAdd(_self, value);
        public void Sub(int value) => _fnSub(_self, value);
        public uint AppendChars(string str) => _fnAppendChars(_self, str);
        public void PrintChars() => _fnPrintChars(_self);
    }
}
