from flask import Flask, request, send_file, jsonify, render_template
import os

app = Flask(__name__)

# Simüle edilmiş kök dizin yapılandırması
BASE_DIR = os.path.dirname(os.path.abspath(__file__))
FAKE_ROOT = os.path.join(BASE_DIR, 'fake_root')
THEMES_DIR = os.path.join(FAKE_ROOT, 'var', 'hyperos', 'themes')

@app.route('/')
def index():
    # Artık düz metin yerine, şık HTML arayüzünü sunuyoruz
    return render_template('index.html')

@app.route('/api/themes/download', methods=['GET'])
def download_theme():
    """
    [VULNERABLE ENDPOINT] - CVE-2025-2844 Simulation
    This endpoint fails to sanitize user input against directory traversal sequences.
    """
    theme_name = request.args.get('theme')
    
    if not theme_name:
        return jsonify({"error": "Missing 'theme' parameter in request."}), 400
        
    # VULNERABILITY: Directly joining path without sanitizing '../'
    # Security Flaw: Lack of input validation allows escaping THEMES_DIR
    file_path = os.path.join(THEMES_DIR, theme_name)
    
    if not os.path.exists(file_path):
        return jsonify({"error": "File not found.", "path": file_path}), 404
        
    try:
        return send_file(file_path, as_attachment=True)
    except Exception as e:
        return jsonify({"error": f"Internal server error: {str(e)}"}), 500

if __name__ == '__main__':
    # Klasörlerin var olduğundan emin ol (Uygulama çökmesini engellemek için)
    os.makedirs(THEMES_DIR, exist_ok=True)
    print(f"[*] Starting simulated HyperOS Theme Manager...")
    print(f"[*] Vulnerable Endpoint: http://127.0.0.1:5000/api/themes/download?theme=default.theme")
    app.run(host='127.0.0.1', port=5000)
