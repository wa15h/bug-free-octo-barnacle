// W0-10 wire-format spike (ADR 0004). Lives only on claude/spike-wire-format; never merged.
using Google.FlatBuffers;

namespace Spike
{
    public static class SnapshotDecoder
    {
        // One decode: wrap the received bytes, then read every field of every entity in list order
        // into the same checksum the Rust generator prints.
        public static ulong Checksum(byte[] bytes)
        {
            var snapshot = Snapshot.GetRootAsSnapshot(new ByteBuffer(bytes));
            ulong c = 0;
            int n = snapshot.EntitiesLength;
            for (int i = 0; i < n; i++)
            {
                var e = snapshot.Entities(i).Value;
                unchecked
                {
                    c = c * 31 + e.Id;
                    c = c * 31 + (ulong)e.V0;
                    c = c * 31 + (ulong)e.V1;
                    c = c * 31 + (ulong)e.V2;
                    c = c * 31 + (ulong)e.V3;
                    c = c * 31 + (ulong)e.V4;
                }
            }
            return c;
        }
    }
}
