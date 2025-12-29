import clsx from 'clsx';
import Link from '@docusaurus/Link';
import Layout from '@theme/Layout';
import Heading from '@theme/Heading';
import styles from './index.module.css';

export default function Home() {
  return (
    <Layout
      title="Sample App Documentation"
      description="Documentation for the Sample Application">
      <main>
        <div className={styles.hero}>
          <Heading as="h1" className="hero__title">
            Sample App Documentation
          </Heading>
          <p className="hero__subtitle">
            Complete documentation for the Sample Application
          </p>
          <div className={styles.buttons}>
            <Link
              className="button button--secondary button--lg"
              to="/docs/intro">
              Get Started
            </Link>
          </div>
        </div>
      </main>
    </Layout>
  );
}

