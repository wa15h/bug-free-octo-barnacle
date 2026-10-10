// W0-10 wire-format spike (ADR 0004). Lives only on claude/spike-wire-format; never merged.
// Reads the Rust generator's snapshots from the directory given as the one argument, times the
// decode in five runs of 10,000 decodes after 1,000 warm-up, checks that each run's decode
// checksums add up to the Rust checksum times the decode count, and prints the medians as the
// table ADR 0004 cites. Exits 1 on a mismatch, which voids the run.
using System;
using System.Diagnostics;
using System.Globalization;
using System.IO;
using System.Linq;
using Spike;

const int Warmup = 1_000, Timed = 10_000, Runs = 5;
var dir = args[0];
var inv = CultureInfo.InvariantCulture;
var ok = true;
var rows = new System.Collections.Generic.List<string>();

foreach (var size in new[] { "cut", "4x" })
{
    var bytes = File.ReadAllBytes(Path.Combine(dir, size + ".bin"));
    // entities bytes encode_us checksum, written by the Rust generator.
    var meta = File.ReadAllText(Path.Combine(dir, size + ".txt")).Trim().Split(' ');
    var expected = ulong.Parse(meta[3], inv);
    var t = new double[Runs];
    var a = new double[Runs];
    for (int r = 0; r < Runs; r++)
    {
        ulong sink = 0;
        for (int i = 0; i < Warmup; i++) sink += SnapshotDecoder.Checksum(bytes);
        long allocated0 = GC.GetAllocatedBytesForCurrentThread();
        long ticks0 = Stopwatch.GetTimestamp();
        for (int i = 0; i < Timed; i++) sink += SnapshotDecoder.Checksum(bytes);
        long ticks1 = Stopwatch.GetTimestamp();
        long allocated1 = GC.GetAllocatedBytesForCurrentThread();
        if (sink != unchecked(expected * (ulong)(Warmup + Timed))) ok = false;
        t[r] = (ticks1 - ticks0) * 1e6 / Stopwatch.Frequency / Timed;
        a[r] = (double)(allocated1 - allocated0) / Timed;
        Console.WriteLine(string.Format(inv, "c# {0} run {1}: t {2:F2} us, a {3:F2} bytes", size, r + 1, t[r], a[r]));
    }
    double Median(double[] xs) => xs.OrderBy(x => x).ElementAt(xs.Length / 2);
    rows.Add(string.Format(inv, "| {0} | {1} | {2} | {3} | {4:F1} | {5:F0} |",
        size, meta[0], meta[1], meta[2], Median(t), Median(a)));
}

Console.WriteLine(ok ? "checksums: every decode matched the Rust checksum" : "checksums: MISMATCH, run void");
Console.WriteLine();
Console.WriteLine("| Size | Entities | Bytes | Rust encode us | C# decode us t | C# bytes allocated a |");
Console.WriteLine("|---|---|---|---|---|---|");
rows.ForEach(Console.WriteLine);
return ok ? 0 : 1;
