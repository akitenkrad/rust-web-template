import './App.css';
import Header from './components/header/Header';

function App() {
  return (
    <div className="App">
      <Header />
      <main className="main-content">
        <section id="home" className="section">
          <h2>ホーム</h2>
          <p>Rust Web Templateへようこそ。このアプリケーションは、RustとReactを使用して構築されています。</p>
        </section>
        
        <section id="about" className="section">
          <h2>概要</h2>
          <p>このプロジェクトは、モダンなWebアプリケーション開発のためのテンプレートです。</p>
        </section>
        
        <section id="services" className="section">
          <h2>サービス</h2>
          <p>高性能なRustバックエンドと、レスポンシブなReactフロントエンドを提供します。</p>
        </section>
        
        <section id="contact" className="section">
          <h2>お問い合わせ</h2>
          <p>ご質問やご提案がございましたら、お気軽にお問い合わせください。</p>
        </section>
      </main>
    </div>
  );
}

export default App;
