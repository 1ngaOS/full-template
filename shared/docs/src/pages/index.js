import Layout from '@theme/Layout';
import Heading from '@theme/Heading';
import styles from './index.module.css';

export default function Home() {
  return (
    <Layout
      title="Shared Services Documentation"
      description="Unified documentation for shared services">
      <main>
        <div className={styles.hero}>
          <Heading as="h1" className="hero__title">
            Shared Services Documentation
          </Heading>
          <p className="hero__subtitle">
            Unified documentation for all shared services
          </p>
        </div>
      </main>
    </Layout>
  );
}

