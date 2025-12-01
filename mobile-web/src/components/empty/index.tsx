import React, { ReactElement } from 'react';
import { ErrorBlock } from 'antd-mobile';
import emptySvg from '@/assets/img/empty.svg';

interface EmptyProps {
  className?: string;
  title?: React.ReactNode;
  description?: React.ReactNode;
  image?: string | ReactElement;
  style?: React.CSSProperties;
}

const Empty: React.FC<EmptyProps> = ({ className, title = '暂无数据', description = '', image, style: style }) => {
  let imageNode: string | ReactElement = emptySvg;
  if (image) {
    imageNode = image;
  }

  return (
    <ErrorBlock
      className={className}
      status='empty'
      title={title}
      description={description}
      image={imageNode}
      style={style}
    />
  );
};

export default Empty;
