import javax.sound.sampled.*;
import java.io.File;

public class PlayWav {
    public static void main(String[] args) {
        try {
            if (args.length < 1) {
                System.err.println("Usage: PlayWav <wav-file>");
                System.exit(1);
            }

            File wavFile = new File(args[0]);
            if (!wavFile.exists()) {
                System.err.println("File not found: " + args[0]);
                System.exit(1);
            }

            System.out.println("Playing: " + wavFile.getName());

            System.out.println("Step 1: Getting AudioInputStream...");
            AudioInputStream audioInputStream = AudioSystem.getAudioInputStream(wavFile);
            AudioFormat format = audioInputStream.getFormat();
            System.out.println("Step 2: Format: " + format);
            System.out.println("  Encoding: " + format.getEncoding());
            System.out.println("  Sample Rate: " + format.getSampleRate());
            System.out.println("  Sample Size: " + format.getSampleSizeInBits());
            System.out.println("  Channels: " + format.getChannels());
            System.out.println("  Frame Size: " + format.getFrameSize());

            System.out.println("Step 3: Getting MixerInfo...");
            try {
                Mixer.Info[] infos = AudioSystem.getMixerInfo();
                System.out.println("  Found " + infos.length + " mixer(s)");
                for (int i = 0; i < infos.length; i++) {
                    System.out.println("  Mixer " + i + ": " + infos[i].getName() + " - " + infos[i].getDescription());
                }
            } catch (Throwable t) {
                System.err.println("MixerInfo error: " + t.getClass().getName() + ": " + t.getMessage());
                Throwable cause = t.getCause();
                while (cause != null) {
                    System.err.println("  Caused by: " + cause.getClass().getName() + ": " + cause.getMessage());
                    cause = cause.getCause();
                }
            }

            System.out.println("Step 4: Getting Mixer...");
            try {
                Mixer.Info[] infos = AudioSystem.getMixerInfo();
                if (infos.length == 0) {
                    System.err.println("No mixers found!");
                    return;
                }
                System.out.println("  Using mixer: " + infos[0].getName());
                Mixer mixer = AudioSystem.getMixer(infos[0]);
                System.out.println("  Mixer class: " + mixer.getClass().getName());
                System.out.println("Step 5: Getting source line info...");
                Line.Info[] sourceLines = mixer.getSourceLineInfo();
                System.out.println("  Source lines: " + sourceLines.length);
                for (int i = 0; i < sourceLines.length; i++) {
                    System.out.println("    Line " + i + ": " + sourceLines[i]);
                }
                System.out.println("Step 6a: Creating DataLine.Info...");
                DataLine.Info clipInfo = new DataLine.Info(Clip.class, format);
                System.out.println("Step 6b: Clip info: " + clipInfo);
                System.out.println("Step 6c: isLineSupported: " + mixer.isLineSupported(clipInfo));
                System.out.println("Step 6d: Getting line...");
                Clip clip = (Clip) mixer.getLine(clipInfo);
                System.out.println("Step 7: Got Clip: " + clip);
                try {
                    clip.open(audioInputStream);
                } catch (Throwable t2) {
                    System.err.println("clip.open failed: " + t2.getClass().getName() + ": " + t2.getMessage());
                    throw t2;
                }
                System.out.println("Step 8: Opened clip");
                clip.start();
                System.out.println("Step 9: Started playback");
                long microseconds = clip.getMicrosecondLength();
                System.out.println("  Clip duration (microseconds): " + microseconds);
                long waitMs = microseconds > 0 ? microseconds / 1000 + 500 : 3000;
                System.out.println("  Waiting " + waitMs + "ms for playback...");
                Thread.sleep(waitMs);
                clip.close();
                audioInputStream.close();
                System.out.println("Playback complete.");
            } catch (Throwable t) {
                System.err.println("Error: " + t.getClass().getName() + ": " + t.getMessage());
                t.printStackTrace(System.err);
                Throwable cause = t.getCause();
                while (cause != null) {
                    System.err.println("  Caused by: " + cause.getClass().getName() + ": " + cause.getMessage());
                    cause = cause.getCause();
                }
            }
        } catch (Throwable t) {
            System.err.println("Error: " + t.getClass().getName() + ": " + t.getMessage());
            Throwable cause = t.getCause();
            while (cause != null) {
                System.err.println("  Caused by: " + cause.getClass().getName() + ": " + cause.getMessage());
                cause = cause.getCause();
            }
        }
    }
}
