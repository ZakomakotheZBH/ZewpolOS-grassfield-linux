const express = require('express');
const { exec } = require('child_process');
const fs = require('fs');
const path = require('path');
const app = express();
const PORT = 3000;

app.use(express.json());

// 1. Serve your ZewpolOS UI
app.get('/', (req, res) => {
    res.sendFile(path.join(__dirname, 'index.html'));
});

// 2. FILE SYSTEM BRIDGE: Reads the actual Linux files
app.get('/api/files', (req, res) => {
    // Reads the directory where the server is running
    fs.readdir(__dirname, (err, files) => {
        if (err) {
            return res.status(500).json({ error: err.message });
        }
        res.json({ files });
    });
});

// 3. TERMINAL BRIDGE: Runs real bash commands on the kernel
app.post('/api/terminal', (req, res) => {
    const command = req.body.command;
    
    exec(command, (error, stdout, stderr) => {
        if (error) {
            return res.json({ output: stderr || error.message });
        }
        res.json({ output: stdout });
    });
});

app.listen(PORT, () => {
    console.log(`ZewpolOS Grassmeadow running at http://localhost:${PORT}`);
});