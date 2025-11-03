import React, { ReactElement } from 'react';
import { ErrorBlock } from 'antd-mobile';
import emptySvg from '@/assets/img/empty.svg';

interface EmptyProps {
  title?: React.ReactNode;
  description?: React.ReactNode;
  image?: string | ReactElement;
  style?: React.CSSProperties;
}

const Empty: React.FC<EmptyProps> = ({ title = '暂无数据', description = '', image, style: style }) => {
  let imageNode: string | ReactElement = emptySvg;
  if (image) {
    imageNode = image;
  }

  return <ErrorBlock status='empty' title={title} description={description} image={imageNode} style={style} />;
};

export default Empty;
