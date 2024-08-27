const express = require('express');
const { exec } = require('child_process');
const fs = require('fs');
const path = require('path');

const app = express();
const port = 3000;

app.use(express.json());
app.use(express.static(path.join(__dirname, 'public')));

// Serve the index.html file
app.get('/', (req, res) => {
    res.sendFile(path.join(__dirname, 'public', 'index.html'));
});

app.post('/generate-witness', (req, res) => {
    const inputX = req.body.X;

    // Save input.json file
    const inputFilePath = path.join(__dirname, 'circuits', 'input.json');
    fs.writeFileSync(inputFilePath, JSON.stringify({ X: inputX }, null, 2));

    // Execute the bash script
    exec(`./compile_circuit.sh`, { cwd: path.join(__dirname, 'circuits') }, (error, stdout, stderr) => {
        if (error) {
            console.error(`Error executing script: ${error}`);
            return res.status(500).send('Error generating witness');
        }

        // Read the witness file
        const witnessFilePath = path.join(__dirname, 'circuits', 'witness.wtns');
        const witnessData = fs.readFileSync(witnessFilePath, 'utf8');
        res.json({ witness: witnessData });
    });
});

app.listen(port, () => {
    console.log(`Server running at http://localhost:${port}`);
});
