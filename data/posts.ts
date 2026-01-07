export interface Post {
    id: string;
    title: string;
    date: string;
    tags: string[];
    preview: string;
    content: string; // HTML or Markdown string for the full post
}

export const posts: Post[] = [
    {
        id: "debugging-vga-neurokernel",
        title: "Debugging the VGA Buffer in NeuroKernel",
        date: "2026-01-04",
        tags: ["Systems", "Assembly", "Visualization"],
        preview: "Why my neural network weights looked like static noise on the screen.",
        content: `
      <h2>The Problem</h2>
      <p>When training the NeuroKernel, I wanted to visualize the weights updating in real-time. I mapped the VGA memory buffer at <code>0xB8000</code> but all I got was garbage.</p>
      
      <h3>The Fix</h3>
      <p>I realized I was writing to the text-mode buffer instead of switching to Mode 13h for graphical output. A quick BIOS interrupt call <code>int 0x10</code> fixed it.</p>
      
      <pre><code>
mov ax, 0x13
int 0x10
      </code></pre>
      
      <p>Now I see the weights converging beautifully.</p>
    `
    }
];
