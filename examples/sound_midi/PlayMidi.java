import javax.sound.midi.MidiDevice;
import javax.sound.midi.MidiSystem;
import javax.sound.midi.Receiver;
import javax.sound.midi.Sequence;
import javax.sound.midi.Sequencer;
import java.io.File;

public class PlayMidi {
    public static void main(String[] args) {
        try {
            if (args.length < 1) {
                System.err.println("Usage: PlayMidi <midi-file>");
                System.exit(1);
            }

            File midiFile = new File(args[0]);
            if (!midiFile.exists()) {
                System.err.println("File not found: " + args[0]);
                System.exit(1);
            }

            System.out.println("Playing: " + midiFile.getName());

            System.out.println("Step 1: Getting sequence...");
            Sequence sequence = MidiSystem.getSequence(midiFile);
            System.out.println("Step 2: Got sequence, duration = " + sequence.getMicrosecondLength() + " microseconds");

            System.out.println("Step 3: Getting sequencer...");
            Sequencer sequencer = MidiSystem.getSequencer(false);

            System.out.println("Step 4: Opening sequencer...");
            sequencer.open();

            // Find a MIDI output device (not sequencer, not synthesizer)
            System.out.println("Step 5: Finding MIDI output device...");
            MidiDevice.Info[] infos = MidiSystem.getMidiDeviceInfo();
            System.out.println("  Found " + infos.length + " MIDI device(s)");
            Receiver receiver = null;
            for (MidiDevice.Info info : infos) {
                MidiDevice device = MidiSystem.getMidiDevice(info);
                System.out.println("  Device: " + info.getName() + " [" + device.getClass().getName() + "]");
                if (!(device instanceof Sequencer) && !(device instanceof javax.sound.midi.Synthesizer) && device.getMaxReceivers() != 0) {
                    try {
                        device.open();
                        receiver = device.getReceiver();
                        System.out.println("  -> Using: " + info.getName());
                        break;
                    } catch (Exception e) {
                        System.out.println("  -> Skipped: " + e.getMessage());
                    }
                }
            }

            if (receiver != null) {
                sequencer.getTransmitter().setReceiver(receiver);
            } else {
                System.out.println("  No MIDI output device found, playing without output");
            }

            System.out.println("Step 6: Setting sequence...");
            sequencer.setSequence(sequence);

            System.out.println("Step 7: Starting playback...");
            sequencer.start();

            // Wait for playback to complete with a timeout
            long maxWaitMs = sequence.getMicrosecondLength() / 1000 + 5000;
            long waited = 0;
            while (sequencer.isRunning() && waited < maxWaitMs) {
                Thread.sleep(100);
                waited += 100;
            }

            sequencer.close();
            System.out.println("Playback complete.");
        } catch (Throwable t) {
            t.printStackTrace();
            System.exit(1);
        }
    }
}
